pub fn exemplo(){
    //tupla é imutável
    let tupla = (1,2,3);
    println!("tupla => {:?}", tupla);
    println!("tupla.0 => {}", tupla.0);

    let (a,b,c) = tupla;
    println!("elementos da tupla => {} {} {}", a, b, c);

    //array de inteiro de 32 bits com 10 posições
    let mut array : [i32; 10] = [0; 10];
    println!("array => {:?}", array);

    array[0] = 1;
    array[3] = 4;
    array[9] = 10;
    println!("array => {:?}", array);

    //pegando partes de um array (slices)
    let mut slice: &[i32] = &array[1..4];
    println!("slice => {:?}", slice);

    slice = &array[2..5];
    slice.iter().for_each(|x| println!("{} ", x));
    println!();

    //vector é algo que pode crescer ou diminuir de tamanho. É dinamico
    let mut vec = vec![7,8,9];
    vec.push(10);
    println!("vec[0] => {}", vec[0]);
    println!("vec.pop() => {}", vec.pop().unwrap());
}