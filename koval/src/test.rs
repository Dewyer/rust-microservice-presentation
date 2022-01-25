#[cfg(test)]
mod tests {
    use std::Arc::Arc;
    use crate::{Container, Injectable};

    trait ServiceTrait {
        fn do_something(&self) -> i32;
    }

    struct ServiceImpl {
    }

    impl ServiceTrait for ServiceImpl {
        fn do_something(&self) -> i32 {
            10
        }
    }

    impl Injectable<Arc<dyn ServiceTrait>> for ServiceImpl {
        fn resolve_injectable(container: &Container) -> Result<Arc<dyn ServiceTrait>> {
            Ok(Arc::new(Self {}))
        }
    }


    #[test]
    fn it_works_unbuilt() {
        let mut container = Container::new()
            .bind_singleton::<Arc<dyn ServiceTrait>, ServiceImpl>();

        let srv = container.resolve::<Arc<dyn ServiceTrait>>()
            .expect("rip lol");

        assert_eq!(srv.do_something(),10);
    }

    #[test]
    fn it_works_built() {
        let mut container = Container::new()
            .bind_singleton::<Arc<dyn ServiceTrait>, ServiceImpl>()
            .build();

        let srv = container.resolve::<Arc<dyn ServiceTrait>>()
            .expect("rip lol");

        assert_eq!(srv.do_something(),10);
    }

    #[test]
    fn it_works_built_w_child() {
        let mut child_container = Container::new()
            .bind_singleton::<Arc<dyn ServiceTrait>, ServiceImpl>();

        let mut container = Container::new()
            .bind_container_into(child_container)
            .build();

        let srv = container.resolve::<Arc<dyn ServiceTrait>>()
            .expect("rip lol");

        assert_eq!(srv.do_something(),10);
    }
}
