use table::Column;
use query::Operand;
use query::operand::ToOperand;
use query::ColumnName;


#[derive(Debug)]
#[derive(Clone)]
pub enum Direction {
    ASC,
    DESC,
}


#[derive(Debug)]
#[derive(Clone)]
pub enum NullsWhere {
    FIRST,
    LAST,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Order {
    pub operand: Operand,
    pub direction: Option<Direction>,
    pub nulls_where: Option<NullsWhere>,
}

impl Order {
    pub fn NULLS_FIRST(mut self) -> Order {
        self.nulls_where = Some(NullsWhere::FIRST);
        self
    }
    pub fn NULLS_LAST(mut self) -> Order {
        self.nulls_where = Some(NullsWhere::FIRST);
        self
    }
}

pub trait ToOrder {
    fn to_order(&self) -> Vec<Order>;
}


macro_rules! impl_to_order_for_order{
	($x:expr) => (
		impl ToOrder for [Order;$x]{
			fn to_order(&self)->Vec<Order>{
				let mut orders = vec![];
				for o in self{
					orders.push(o.to_owned())
				}
				orders
			}
		}
	);
}

impl_to_order_for_order!(1);
impl_to_order_for_order!(2);
impl_to_order_for_order!(3);
impl_to_order_for_order!(4);
impl_to_order_for_order!(5);
impl_to_order_for_order!(6);
impl_to_order_for_order!(7);
impl_to_order_for_order!(8);
impl_to_order_for_order!(9);
impl_to_order_for_order!(10);
impl_to_order_for_order!(11);
impl_to_order_for_order!(12);


pub trait HasDirection {
    fn ASC(&self) -> Order;
    fn DESC(&self) -> Order;
    fn ASC_NULLS_FIRST(self) -> Order;
    fn ASC_NULLS_LAST(self) -> Order;
    fn DESC_NULLS_FIRST(self) -> Order;
    fn DESC_NULLS_LAST(self) -> Order;
}

impl<T> HasDirection for T
    where T: ToOperand
{
    fn ASC(&self) -> Order {
        Order {
            operand: self.to_operand(),
            direction: Some(Direction::ASC),
            nulls_where: None,
        }
    }

    fn DESC(&self) -> Order {
        Order {
            operand: self.to_operand(),
            direction: Some(Direction::DESC),
            nulls_where: None,
        }
    }

    fn ASC_NULLS_FIRST(self) -> Order {
        self.ASC().NULLS_FIRST()
    }

    fn ASC_NULLS_LAST(self) -> Order {
        self.ASC().NULLS_LAST()
    }

    fn DESC_NULLS_FIRST(self) -> Order {
        self.DESC().NULLS_FIRST()
    }

    fn DESC_NULLS_LAST(self) -> Order {
        self.DESC().NULLS_LAST()
    }
}
