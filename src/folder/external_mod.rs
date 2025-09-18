
pub mod section{
    #[derive(Debug)]
    pub struct Retangle{
        witdh: i32,
        hight: i32,
    }

    impl Retangle {
        pub fn create(x:i32,y:i32)-> Self{
            Self{
                witdh: x,
                hight: y
            }
        }
    }
}