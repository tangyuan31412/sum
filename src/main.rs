fn sumvec(vecu32: &[u32]) -> u32{   
    // vecu32.iter().sum()
    vecu32.iter().sum()
}

fn main() {
    let vec: Vec<u32> = Vec::new();
    let a = vec![0, 1, 2, 3, 4, 5];
    // let mut a:Vec<u32>=[1,2,8,9];
    let s: u32 = sumvec(&a);
    print!("{}", s)
}




// use std::io;
// use std::collections::HashMap;
 
// fn main() {
//     let mut vt:Vec<i32>=Vec::new();
//     println!("Please input the size of the array");
//     let k=read_num();
//     println!("Input the {} members of the array,each number on one line",k);
//     let mut sum_x=0;
//     for t in 0..k{
//         vt.push(read_num());
//         sum_x+=&vt[t as usize];
//     }
 
//     vt.sort();
//     let d=(k/2) as usize;
//     let median=match k%2 {
//        1 => vt[d] as f64,
//         _ => {
//             let p=d-1;
//             (vt[p]+vt[d]) as f64/2.0
//         }
//     };
 
//     let mut mode:Option<Vec<i32>>=Option::None;
//     let mut hash_m=HashMap::new();
//     for s in &vt{
//         let count=hash_m.entry(s).or_insert(0);
//         *count+=1;
//     }
//     let mut mode_bak_vec=Vec::new();
//     let mut max_num=0;
//     for i in &hash_m{
//         if max_num==*i.1
//         {
//             mode_bak_vec.push(**i.0);
//         }else if max_num<*i.1 {
//             mode_bak_vec=Vec::new();
//             mode_bak_vec.push(**i.0);
//             max_num=*i.1;
//         }
//     }
//     if mode_bak_vec.len()!=hash_m.len() as usize
//     {
//         mode=Option::Some(mode_bak_vec);
//     }
 
//     let avg_num=sum_x as f64 / k as f64;
 
//     println!("The sorted arr is {:?}",vt);
//     println!("The hash_map is {:?}",hash_m);
//     println!("The median is {}",median);
//     println!("The modes are {:?}",mode);
//     println!("The average is {}",avg_num);
// }
 
// fn read_num()->i32
// {
//     let mut s=String::new();
//     io::stdin().read_line(&mut s).expect("raed error");
//     s.trim().parse().expect("not a num")
// }
