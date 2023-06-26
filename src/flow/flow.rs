pub mod sync {
    use std::{collections::HashMap, hash::Hash};

    pub type TProcessor<V> = fn(&V) -> Option<&V>;
    pub type TFilter<V> = fn(&V) -> bool;
    pub type TObserver<V> = fn(&V);
    pub type TMapper<VFrom, VTo> = fn(&VFrom) -> Option<&VTo>;
    pub type TClassificator<V, C> = fn(&V) -> Vec<C>;

    trait Processor: 'static {
        type V;

        fn execute<'a>(&'a self, value: &'a Self::V) -> Option<&'a Self::V>;
        fn outputs<'a>(&'a mut self) -> Option<&'a mut [Flow<Self::V>]> {
            None
        }
    }

    pub struct Flow<V>
    where
        V: 'static,
    {
        next: Vec<Flow<V>>,
        processor: Box<dyn Processor<V = V>>,
    }

    impl<V> Flow<V>
    where
        V: 'static,
    {
        fn _attach<P>(&mut self, p: P) -> &mut Flow<V>
        where
            P: Processor<V = V> + 'static,
        {
            let f = Flow {
                next: Vec::new(),
                processor: Box::new(p),
            };
            self.next.push(f);
            self.next.last_mut().unwrap()
        }

        pub fn next(&mut self, processor: &'static TProcessor<V>) -> &mut Flow<V> {
            return self._attach(GeneralProcessor::<V>::new(processor));
        }

        pub fn send(&self, value: &V) {
            let result = self.processor.execute(value);
            if result.is_some() {
                let result_value = &result.unwrap();
                for n in &self.next {
                    n.send(*result_value);
                }
            }
        }

        pub fn filter(&mut self, filter: &'static TFilter<V>) -> &mut Flow<V> {
            return self._attach(Filter::<V>::new(filter));
        }

        pub fn peep(&mut self, observer: &'static TObserver<V>) -> &mut Flow<V> {
            return self._attach(Observer::<V>::new(observer));
        }
    }

    pub fn new_flow<V>() -> Flow<V>
    where
        V: 'static,
    {
        Flow {
            next: Vec::new(),
            processor: Box::new(GeneralProcessor::<V>::new(&(pass as TProcessor<V>))),
        }
    }

    fn pass<V>(val: &V) -> Option<&V> {
        Some(val)
    }

    pub fn send_many<T, I>(flow: &Flow<T>, mut iterator: I)
    where
        T: 'static,
        I: Iterator<Item = T>,
    {
        loop {
            let i = iterator.next();
            if i.is_some() {
                let i_val = i.unwrap();
                flow.send(&i_val);
            } else {
                break;
            }
        }
    }

    pub fn segregate<'a, V, C>(
        flow: &'a mut Flow<V>,
        classificator: &'static TClassificator<V, C>,
        classes: Vec<C>,
    ) -> &'a mut [Flow<V>]
    where
        V: 'static,
        C: PartialEq + Eq + Hash + Copy + 'static,
    {
        let n = flow._attach(Classificator::<V, C>::new(classificator, &classes));
        n.processor.outputs().unwrap()
    }

    struct GeneralProcessor<V>
    where
        V: 'static,
    {
        processor: &'static TProcessor<V>,
    }

    impl<V> Processor for GeneralProcessor<V>
    where
        V: 'static,
    {
        type V = V;

        fn execute<'a>(&'a self, value: &'a V) -> Option<&'a V> {
            (*self.processor)(value)
        }
    }

    impl<V> GeneralProcessor<V>
    where
        V: 'static,
    {
        fn new(processor: &'static TProcessor<V>) -> GeneralProcessor<V> {
            return GeneralProcessor { processor };
        }
    }

    struct Filter<V>
    where
        V: 'static,
    {
        filter: &'static TFilter<V>,
    }

    impl<V> Processor for Filter<V>
    where
        V: 'static,
    {
        type V = V;

        fn execute<'a>(&'a self, value: &'a V) -> Option<&'a V> {
            if (*self.filter)(value) {
                return Some(value);
            }
            None
        }
    }

    impl<V> Filter<V>
    where
        V: 'static,
    {
        fn new(filter: &'static TFilter<V>) -> Filter<V> {
            return Filter { filter };
        }
    }

    struct Observer<V>
    where
        V: 'static,
    {
        observer: &'static TObserver<V>,
    }

    impl<V> Processor for Observer<V>
    where
        V: 'static,
    {
        type V = V;
        fn execute<'a>(&'a self, value: &'a V) -> Option<&'a V> {
            (*self.observer)(value);
            Some(value)
        }
    }

    impl<V> Observer<V>
    where
        V: 'static,
    {
        fn new(observer: &'static TObserver<V>) -> Observer<V> {
            return Observer { observer };
        }
    }

    struct Classificator<V, C>
    where
        V: 'static,
        C: PartialEq + Eq + Hash + 'static,
    {
        classify: &'static TClassificator<V, C>,
        class_map: HashMap<C, usize>,
        flows: Vec<Flow<V>>,
    }

    impl<V, C> Processor for Classificator<V, C>
    where
        V: 'static,
        C: PartialEq + Eq + Hash + 'static,
    {
        type V = V;

        fn execute(&self, value: &V) -> Option<&V> {
            let classes = (*self.classify)(value);
            for class in &classes {
                let flow_idx = self.class_map.get(class).unwrap();
                self.flows[*flow_idx].send(value);
            }
            None // flow stops here
        }

        fn outputs(&mut self) -> Option<&mut [Flow<V>]> {
            return Some(&mut self.flows);
        }
    }
    impl<V, C> Classificator<V, C>
    where
        V: 'static,
        C: PartialEq + Eq + Hash + Copy + 'static,
    {
        fn new(
            classificator: &'static TClassificator<V, C>,
            classes: &Vec<C>,
        ) -> Classificator<V, C> {
            let mut map = HashMap::<C, usize>::new();
            let mut flows = Vec::<Flow<V>>::new();

            for i in 0..classes.len() {
                flows.push(new_flow());
                map.insert(classes[i], i);
            }

            Classificator {
                classify: classificator,
                class_map: map,
                flows,
            }
        }
    }
}
