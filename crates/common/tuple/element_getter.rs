/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use butterscotch_codegen::generate_tuple_impls;

pub trait TupleElementGetter {
    type Item0 = ();
    type Item1 = ();
    type Item2 = ();
    type Item3 = ();
    type Item4 = ();
    type Item5 = ();
    type Item6 = ();
    type Item7 = ();
    type Item8 = ();
    type Item9 = ();

    fn get_0(&self) -> &Self::Item0 { unimplemented!(); }
    fn get_1(&self) -> &Self::Item1 { unimplemented!(); }
    fn get_2(&self) -> &Self::Item2 { unimplemented!(); }
    fn get_3(&self) -> &Self::Item3 { unimplemented!(); }
    fn get_4(&self) -> &Self::Item4 { unimplemented!(); }
    fn get_5(&self) -> &Self::Item5 { unimplemented!(); }
    fn get_6(&self) -> &Self::Item6 { unimplemented!(); }
    fn get_7(&self) -> &Self::Item7 { unimplemented!(); }
    fn get_8(&self) -> &Self::Item8 { unimplemented!(); }
    fn get_9(&self) -> &Self::Item9 { unimplemented!(); }

    fn get_mut_0(&mut self) -> &mut Self::Item0 { unimplemented!(); }
    fn get_mut_1(&mut self) -> &mut Self::Item1 { unimplemented!(); }
    fn get_mut_2(&mut self) -> &mut Self::Item2 { unimplemented!(); }
    fn get_mut_3(&mut self) -> &mut Self::Item3 { unimplemented!(); }
    fn get_mut_4(&mut self) -> &mut Self::Item4 { unimplemented!(); }
    fn get_mut_5(&mut self) -> &mut Self::Item5 { unimplemented!(); }
    fn get_mut_6(&mut self) -> &mut Self::Item6 { unimplemented!(); }
    fn get_mut_7(&mut self) -> &mut Self::Item7 { unimplemented!(); }
    fn get_mut_8(&mut self) -> &mut Self::Item8 { unimplemented!(); }
    fn get_mut_9(&mut self) -> &mut Self::Item9 { unimplemented!(); }
}

generate_tuple_impls!(8, r"
    impl<%{%TR,%}> TupleElementGetter for (%{%TR,%}) {
        %{type Item%VI = %TR;%}

        %{fn get_%VI(&self) -> &Self::Item%VI {
            &self.%VI
        }%}

        %{fn get_mut_%VI(&mut self) -> &mut Self::Item%VI {
            &mut self.%VI
        }%}
    }
");
