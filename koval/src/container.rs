use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::ops::{Deref};
use crate::Injectable;
use std::sync::Arc;
use crate::InjectionError;

pub type ResolutionFn = Arc<dyn Fn(&Container) -> Result<Arc<dyn Any>, InjectionError>>;

#[derive(Eq, PartialEq, Clone)]
pub enum ResolutionType {
    Singleton,
    Transient,
}

#[derive(Clone)]
pub struct Resolution {
    pub resolution_type: ResolutionType,
    pub stored_instance: Option<Arc<dyn Any>>,
    pub resolution_fn: ResolutionFn,
}

#[derive(Clone)]
pub struct Container {
    bindings: HashMap<TypeId, Resolution>,
}

fn wrap_injectable<T, Fi>(inj_fun: &'static Fi) -> ResolutionFn
where Fi: 'static + Fn(&Container) -> Result<T, InjectionError>, Result<T, InjectionError>: 'static
{
    Arc::new(|cont: &Container| -> Result<Arc<dyn Any>, InjectionError> {
        let resolved = inj_fun(cont)?;

        Ok(Arc::new(resolved))
    })
}

impl Container {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    pub fn bind_singleton<InjTraitType, ActType>(mut self) -> Self
    where InjTraitType: 'static + Sized,
        ActType: Injectable<InjTraitType> + 'static,
    {
        let id = TypeId::of::<InjTraitType>();
        self.bindings.insert(id, Resolution {
            resolution_type: ResolutionType::Singleton,
            stored_instance: None,
            resolution_fn: wrap_injectable( &ActType::resolve_injectable),
        });

        self
    }

    pub fn bind_transient<InjTraitType, ActType>(mut self) -> Self
        where InjTraitType: 'static + Sized,
              ActType: Injectable<InjTraitType> + 'static,
    {
        let id = TypeId::of::<InjTraitType>();
        self.bindings.insert(id, Resolution {
            resolution_type: ResolutionType::Transient,
            stored_instance: None,
            resolution_fn: wrap_injectable( &ActType::resolve_injectable),
        });

        self
    }

    pub fn resolve<InjTraitType>(&self) -> Result<InjTraitType, InjectionError>
    where InjTraitType: 'static + Clone
    {
        let id = TypeId::of::<InjTraitType>();
        let binding = self.bindings.get(&id)
            .ok_or(InjectionError::TypeNotBound)?;


        if binding.resolution_type == ResolutionType::Singleton {
            if let Some(inst) = binding.stored_instance.as_ref() {
                inst.clone()
                    .deref()
                    .downcast_ref::<InjTraitType>()
                    .map(|cc| cc.clone())
                    .ok_or(InjectionError::TypeNotBound)
            } else {
                let resolved = (binding.resolution_fn)(self)?;
                resolved.deref().downcast_ref::<InjTraitType>()
                    .map(|cc| cc.clone())
                    .ok_or(InjectionError::TypeNotBound)
            }
        }
        else if binding.resolution_type == ResolutionType::Transient {
            let resolved = (binding.resolution_fn)(self)?;
            resolved.deref().downcast_ref::<InjTraitType>()
                .map(|cc| cc.clone())
                .ok_or(InjectionError::TypeNotBound)
        }
        else {
            unreachable!();
        }
    }

    pub fn bind_container_into(mut self, container: Self) -> Self {
        for (key, val) in container.bindings.into_iter() {
            self.bindings.insert(key, val);
        }

        self
    }

    pub fn build(mut self) -> Result<Self, InjectionError> {
        let old_instance = self.clone();
        for (id, val) in old_instance.bindings.iter() {
            if val.resolution_type == ResolutionType::Singleton && val.stored_instance.is_none() {
                let instance: Arc<dyn Any> = (val.resolution_fn)(&self)?;
                let in_self = self.bindings.get_mut(&id).unwrap();
                in_self.stored_instance = Some(instance);
            }
        }

        Ok(self)
    }
}