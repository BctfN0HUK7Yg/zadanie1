use std::thread;
use std::time::Duration;

const TRASHHOLD:usize=4;


fn f<T,R>(_t:&T)->R 
    where R:Default {
    thread::sleep(Duration::from_millis(500));
    R::default()
}

fn split_work<T,R> (work:&Vec<T>) -> Vec<R> 
    where T:Sized,T:Sync, R:Default, R:Send, R:Clone {

     
    let mut whole_res=Vec::new(); // for collect results

    let mut ranges_of_work=Vec::new();
    for i in (0..work.len()).step_by(TRASHHOLD) { 

        let size_of_work = if i+TRASHHOLD>work.len() {work.len()-i} else {TRASHHOLD}; //last part should be less then TRASHHOLD
        
        ranges_of_work.push((i,i+size_of_work));
    }
    let num_of_threasds=work.len()/TRASHHOLD+1;

        crossbeam::scope(|s|{
            let mut handles=Vec::new();
            
            for y in 0..num_of_threasds {
                let current_range=ranges_of_work[y];
                let part_of_work=&work[current_range.0..current_range.1]; //get slice of works
                println!("Start thread {} to compute work range from {} to {}",y,current_range.0,current_range.1);
            let handle =s.spawn(move |_| {
                let mut res:Vec<R>=Vec::new();
                
                 for i in 0..part_of_work.len() {
                    println!("\tdoing work in thread {:?} with element {}",y,y*(current_range.1-current_range.0)+ i);
                    res.push(f(&part_of_work[i]));
                }
                
                res
            });
            handles.push(handle);
            
        }
        for handle in handles {
            let this_res=handle.join().unwrap();
            whole_res.extend_from_slice(&this_res);
        }
            
        }).unwrap();

   
    
    whole_res
}

fn main() {
    let input=vec![0;30];
    println!("Start compute {} tasks",input.len());
    println!("Trashhold for spawn thread is {} tasks",TRASHHOLD);
    let res=split_work::<_,i32>(&input);
    println!("\nResult: {:?}",res);
    println!("Result length: {:?}",res.len());
}

#[cfg(test)]
mod tests {
    //everywhere output is vec of default values
    #[test]
    fn input_30_i32_output_i32() {
        let input=vec![1;30];
        let res=crate::split_work::<_,i32>(&input);
        let rigth_res=vec![0;30];
        assert_eq!(res, rigth_res);
    }
    #[test]
    fn input_17_f64_output_i32() {
        let input=vec![1.0;30];
        let res=crate::split_work::<_,i32>(&input);
        let rigth_res=vec![0;30];
        assert_eq!(res, rigth_res);
    }
    #[test]
    fn input_50_f64_output_f64() {
        let input=vec![1.0;50];
        let res=crate::split_work::<_,f64>(&input);
        let rigth_res=vec![0.0;50];
        assert_eq!(res, rigth_res);
    }
    #[test]
    fn input_13_str_output_i32() {
        let input=vec!["0";13];
        let res=crate::split_work::<_,i32>(&input);
        let rigth_res=vec![0;13];
        assert_eq!(res, rigth_res);
    }
}

/* Implement basic function to split some generic computational work between threads. 
Split should occur only on some threshold - if computational work (input length) is shorter than this threshold, 
no splitting should occur and no threads should be created.

You get as input: 

1. Vec<T>
2. Function f(t: T) -> R


Threshold can be just constant. 

You should return:
   1. Up to you, but probably some Vec of the same length as input(1)

Code should be published on github.


There should be some tests in repository */