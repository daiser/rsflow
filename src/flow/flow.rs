pub mod sync {
    pub type TProcessor<T> = fn(&T) -> Option<T>;
    pub type TFilter<T> = fn(&T) -> bool;
    pub type TObserver<T> = fn(&T);
    pub type TMapper<TSrc, TDst> = fn(&TSrc) -> TDst;
    pub type TClassificator<TVal, TCls> = fn(&TVal) -> Vec<&TCls>;

    trait Processor<T: Sized>: 'static {
        fn execute(&self, value: &T) -> Option<T>;
    }

    pub struct Flow<T: Clone + Copy> {
        next: Vec<Flow<T>>,
        processor: Box<dyn Processor<T>>,
    }

    impl<T: Clone + Copy + 'static> Flow<T> {
        fn _next(&mut self, p: Box<dyn Processor<T>>) -> &mut Flow<T> {
            let f = Flow {
                next: Vec::new(),
                processor: p,
            };
            self.next.push(f);
            self.next.last_mut().unwrap()
        }

        pub fn next(&mut self, processor: &'static TProcessor<T>) -> &mut Flow<T> {
            return self._next(Box::new(GeneralProcessor { processor }));
        }

        pub fn send(&self, value: &T) {
            let result = self.processor.execute(value);
            if result.is_some() {
                let result_value = &result.unwrap();
                for n in &self.next {
                    n.send(result_value);
                }
            }
        }

        pub fn filter(&mut self, filter_fn: &'static TFilter<T>) -> &mut Flow<T> {
            return self._next(Box::new(Filter { filter: filter_fn }));
        }
    }

    pub fn new_flow<'t, T>() -> Flow<T>
    where
        T: Clone + Copy + 'static,
    {
        Flow {
            next: Vec::new(),
            processor: Box::new(GeneralProcessor {
                processor: &(copy as TProcessor<T>),
            }),
        }
    }

    fn copy<T>(value: &T) -> Option<T>
    where
        T: Copy,
    {
        Some((*value).clone())
    }

    struct GeneralProcessor<'t, T: 'static> {
        processor: &'t TProcessor<T>,
    }

    impl<T: 'static> Processor<T> for GeneralProcessor<'static, T> {
        fn execute(&self, value: &T) -> Option<T> {
            (*self.processor)(value)
        }
    }

    struct Filter<'t, T: 'static> {
        filter: &'t TFilter<T>,
    }

    impl<'t, T: Copy> Processor<T> for Filter<'static, T> {
        fn execute(&self, value: &T) -> Option<T> {
            if (*self.filter)(value) {
                return copy(value);
            }
            None
        }
    }
}
