#![allow(clippy::too_many_arguments)]

use std::cell::{Cell, RefCell};
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};

pub struct ClownCar<T> {
    phantom: PhantomData<T>,
}

impl<T> ClownCar<T> {
    pub fn use_ref(_val: &T) {
        todo!();
    }

    pub fn use_mut_ref(_val: &mut T) {
        todo!();
    }

    pub fn clown_car(
        mut cell: Cell<T>,
        ref_cell: RefCell<T>,
        my_box: Box<T>,
        ref_count: Rc<T>,
        mut ref_count_cell: Rc<Cell<T>>, // extremely limited.  requires Rc::is_unique() ; that this be the singular instance (reference count == 1)
        ref_count_refcell: Rc<RefCell<T>>,
        mutex: Mutex<T>,
        rw_lock: RwLock<T>,
        arc: Arc<T>,
        arc_mutex: Arc<Mutex<T>>,
        arc_rw_lock: Arc<RwLock<T>>,
    ) {
        Self::use_ref(cell.get_mut());
        Self::use_ref(&ref_cell.borrow());
        Self::use_ref(&my_box);
        Self::use_ref(&ref_count);
        Self::use_ref(Rc::get_mut(&mut ref_count_cell).unwrap().get_mut());
        Self::use_ref(&ref_count_refcell.borrow());
        Self::use_ref(&mutex.lock().unwrap());
        Self::use_ref(&rw_lock.read().unwrap());
        Self::use_ref(&arc);
        Self::use_ref(&arc_mutex.lock().unwrap());
        Self::use_ref(&arc_rw_lock.read().unwrap());
    }

    pub fn clown_car_ref(
        cell: &mut Cell<T>,
        ref_cell: &RefCell<T>,
        my_box: &Box<T>,
        ref_count: &Rc<T>,
        ref_count_cell: &mut Rc<Cell<T>>, // extremely limited.  requires Rc::is_unique() ; that this be the singular instance (reference count == 1)
        ref_count_refcell: &Rc<RefCell<T>>,
        mutex: &Mutex<T>,
        rw_lock: &RwLock<T>,
        arc: &Arc<T>,
        arc_mutex: &Arc<Mutex<T>>,
        arc_rw_lock: &Arc<RwLock<T>>,
    ) {
        Self::use_ref(cell.get_mut());
        Self::use_ref(&*ref_cell.borrow());
        Self::use_ref(my_box);
        ClownCar::<T>::use_ref(ref_count);
        Self::use_ref(Rc::get_mut(ref_count_cell).unwrap().get_mut());
        Self::use_ref(&ref_count_refcell.borrow());
        Self::use_ref(&*mutex.lock().unwrap());
        Self::use_ref(&*rw_lock.read().unwrap());
        Self::use_ref(arc);
        Self::use_ref(&*arc_mutex.lock().unwrap());
        Self::use_ref(&*arc_rw_lock.read().unwrap());
    }

    pub fn mut_clown_car(
        mut cell: Cell<T>,
        ref_cell: RefCell<T>,
        mut my_box: Box<T>,
        mut ref_count: Rc<T>, // extremely limited.  requires Rc::is_unique() ; that this be the singular instance (reference count == 1)
        mut ref_count_cell: Rc<Cell<T>>, // extremely limited.  requires Rc::is_unique() ; that this be the singular instance (reference count == 1)
        ref_count_refcell: Rc<RefCell<T>>,
        mutex: Mutex<T>,
        rw_lock: RwLock<T>,
        arc: Arc<T>, // doomed
        arc_mutex: Arc<Mutex<T>>,
        arc_rw_lock: Arc<RwLock<T>>,
    ) {
        Self::use_mut_ref(cell.get_mut());
        Self::use_mut_ref(&mut *ref_cell.borrow_mut());
        Self::use_mut_ref(&mut *my_box);
        Self::use_mut_ref(Rc::get_mut(&mut ref_count).unwrap());
        Self::use_mut_ref(Rc::get_mut(&mut ref_count_cell).unwrap().get_mut());
        Self::use_mut_ref(&mut *(*ref_count_refcell).borrow_mut());
        Self::use_mut_ref(&mut *mutex.lock().unwrap());
        Self::use_mut_ref(&mut *rw_lock.write().unwrap());
        // not possible to get &mut T from Arc<T> Self::use_mut_ref(&mut *arc);
        Self::use_mut_ref(&mut *arc_mutex.lock().unwrap());
        Self::use_mut_ref(&mut *arc_rw_lock.write().unwrap());
    }
}
