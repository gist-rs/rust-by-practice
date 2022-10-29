mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        crate::front_of_house::serving::serve_order();
    }

    fn cook_order() {}
}
