interface IA {

}

interface IA1 extends IA {

}

interface IA2 extends IA1 {

}

interface IA3 extends IA {

}

declare type B = IA1 | IA2 | IA3;

declare type C = string | string | string;