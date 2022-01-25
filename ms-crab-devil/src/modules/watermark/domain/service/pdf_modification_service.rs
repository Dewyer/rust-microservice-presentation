use crate::modules::common::errors::generic::GenericError;
use crate::modules::common::errors::watermark::WatermarkError;
use crate::modules::common::errors::ApiError;
use lopdf::content::{Content, Operation};
use lopdf::{dictionary, Document, Object};
use rusoto_s3::StreamingBody;
use std::time::Instant;
use tokio::task;

pub struct PdfModificationService {}

#[inline]
fn other_error(msg: &str) -> ApiError {
    ApiError::Watermark(WatermarkError::Other(msg.to_string()))
}

pub struct WatermarkPdfPayload {
    pub estimated_size: usize,
    pub byte_stream: StreamingBody,
    pub watermark_text: String,
}

pub struct PdfWatermarkingResult {
    pub pdf_bytes: Vec<u8>,
}

impl PdfModificationService {
    pub async fn watermark_pdf(
        payload: WatermarkPdfPayload,
    ) -> Result<PdfWatermarkingResult, ApiError> {
        task::spawn_blocking(move || {
            let now = Instant::now();
            let mut doc = Document::load_from(payload.byte_stream.into_blocking_read())
                .map_err(|_| ApiError::Watermark(WatermarkError::FailedToLoadDocument))?;
            info!("pdf {}ms - took", now.elapsed().as_millis());

            let font_id = doc.add_object(dictionary! {
                "Type" => "Font",
                "Subtype" => "Type1",
                "BaseFont" => "Courier",
            });
            doc.add_object(dictionary! {
                "Font" => dictionary! {
                    "F1" => font_id,
                },
            });

            info!("pdf {}ms - took", now.elapsed().as_millis());

            for (_, page_id) in doc.get_pages().into_iter() {
                let content_data = doc
                    .get_page_content(page_id)
                    .map_err(|_| other_error("Failed to get page content"))?;
                let content = Content::decode(&content_data)
                    .map_err(|_| other_error("Failed to decode page content"))?;

                let watermark_text = vec![
                    Operation::new("BT", vec![]),
                    Operation::new("Tf", vec!["F1".into(), Object::Integer(100)]),
                    Operation::new(
                        "Tm",
                        vec![
                            1.into(),
                            0.into(),
                            0.into(),
                            Object::Integer(-1),
                            100.into(),
                            300.into(),
                        ],
                    ),
                    Operation::new(
                        "Tj",
                        vec![Object::string_literal(payload.watermark_text.clone())],
                    ),
                    Operation::new("ET", vec![]),
                ];

                let content = Content {
                    operations: vec![content.operations, watermark_text].concat(),
                };

                let modified_content = content
                    .encode()
                    .map_err(|_| other_error("Failed to encode page content"))?;
                doc.change_page_content(page_id, modified_content)
                    .map_err(|_| other_error("Failed to change page content"))?;
            }

            doc.compress();
            let mut out_buff = Vec::new();
            doc.save_to(&mut out_buff)
                .map_err(|_| other_error("Failed to save output file"))?;

            Ok(PdfWatermarkingResult {
                pdf_bytes: out_buff,
            })
        })
        .await
        .map_err(|_| ApiError::Generic(GenericError::BadRequest))?
    }
}
