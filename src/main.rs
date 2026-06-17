fn main() {
    println!("Hello, world!");

    // This is a comment
    // aqui donde escribo code

    //Puedes usar tantas println!() macros como quieras.
    // Hay que tomar en cuenta que se añadirá una nueva línea por cada macro:

    println!("Hello World!");
    println!("I am learning Rust.");
    println!("It is awesome!");

    //También hay una print!() macro, que es similar a println!().
    //La única diferencia es que no inserta una nueva línea al final de la salida:

    print!("Hello World! ");
    print!("I will print on the same line.");

    //
    // Agregar Nuevas lineas manualmente
    //

    //Si realmente quieres añadir una nueva línea en print!(), puedes usar el \n carácter:

    print!("Hello World!\n");
    print!("I will print on the same line.");

    //  Al igual que en muchos otros lenguajes de programación, puedes usar el carácter de nueva línea ( \n) para separar líneas. Es una secuencia de escape que obliga al cursor a cambiar su posición al inicio de la siguiente línea en la pantalla. Esto genera una nueva línea.

    //También puedes dividir una línea en medio de una oración. Esto se aplica tanto a print!()como println!()a:
    println!("Hello World!\nThis line was broken up!");

    //
    // Comentarios sobre Rust
    //

    //Los comentarios de una sola línea comienzan con dos barras inclinadas ( //).

    //Cualquier texto que se encuentre entre //y el final de la línea es ignorado por el compilador (no se ejecutará).
    //Este ejemplo utiliza un comentario de una sola línea antes de una línea de código:

    // This is a comment
    println!("Hello World con comentario simple!");

    //Comentarios de varias líneas

    //Los comentarios de varias líneas comienzan con /*y terminan con */.

    //Cualquier texto entre /*y */será ignorado por el compilador:

    /* The code below will print the words Hello World!
    to the screen, and it is amazing */
    println!("Hello World con comentario de varias líneas!");

    /*
    ¿Comentarios de una sola línea o de varias líneas?
    Depende de ti cuál quieras usar. Normalmente, usamos //para comentarios cortos y /* */para comentarios más largos./
    */

    //
    // variables
    //

    // Las variables son contenedores para almacenar valores de datos, como números y caracteres.
    //Para crear una variable en Rust, utilice la letpalabra clave y especifique el nombre de la variable ( nombre en este ejemplo):
    let name = "Jose";
    println!("My first name is: {}", name);

    /*
        Qué es {}?
    Rust utiliza {}como marcador de posición println!()para mostrar valores de variables.

    En el ejemplo anterior, el resultado será "Mi nombre es: Jose".

    Puedes usar tantos marcadores de posición como quieras:
    */

    let name = "Jose";
    let age = 32;
    println!("{} is {} years old.", name, age);

    /*
    Uso de marcadores de posición en orden
        Cuando se utilizan varios marcadores de posición, los valores que se pasan se utilizan en el mismo orden.

    En el ejemplo anterior:

    El primero {}se reemplaza con name("Jose")
    El segundo {}se reemplaza con age(32)
    Importante: El orden importa. Si cambias los valores, el resultado cambiará:
    */

    //Este ejemplo muestra un orden incorrecto:

    let name = "Jose";
    let age = 30;
    println!("{} is {} years old.", age, name); // Outputs 30 is Jose years old

    //Los valores de las variables no se pueden cambiar por defecto.

    //Por defecto, las variables en Rust no se pueden modificar después de su creación:

    //let x = 5;
    //x = 10; // Error
    //println!("{}", x);

    //Cambiar valores de las variables
    //Si desea cambiar el valor de una variable, debe usar la mutpalabra clave (que significa mutable/cambiable):
    let mut x = 5;
    println!("Before: {}", x);
    x = 10;
    println!("After: {}", x);

    //
    // Tipos de datos
    //

    /*
        A diferencia de muchos otros lenguajes de programación, las variables en Rust no necesitan
        declararse con un tipo específico (como "String" para texto o "Int" para números,
        si estás familiarizado con ellos de C o Java ).
        En Rust, el tipo de una variable viene determinado por el valor que se le asigna.
        Rust analiza el valor y elige automáticamente el tipo correcto:

    Ejemplo: */

    //let my_num = 5; // integer
    //let my_double = 5.99; // float
    //let my_letter = 'D'; // character
    //let my_bool = true; // boolean
    //let my_text = "Hello"; // string

    //Sin embargo, es posible indicarle explícitamente a Rust qué tipo de valor debe tener:

    //let my_num: i32 = 5; // integer
    //let my_double: f64 = 5.99; // float
    //let my_letter: char = 'D'; // character
    //let my_bool: bool = true; // boolean
    //let my_text: &str = "Hello"; // string

    /*
        Más adelante en este tutorial, aprenderás más sobre cuándo debes especificar el tipo. En cualquier caso, es importante comprender qué significan los diferentes tipos.

    Los tipos de datos básicos en Rust se dividen en diferentes grupos:

    Números - Números enteros y números decimales ( i32, f64)
    Caracteres : letras o símbolos individuales ( char)
    Cadenas - Texto, una secuencia de caracteres ( &str)
    Booleanos - Valores verdaderos o falsos ( bool)
    */

    //
    // Numeros
    //

    //Los tipos de números se dividen en dos grupos: tipos enteros y tipos de punto flotante.

    //     Entero (i32)
    // Este i32 tipo de dato se utiliza para almacenar números enteros, positivos o negativos (como 123 o -456), sin decimales:
    let age: i32 = 25;
    print!("Numeros enteros: ");
    println!("Age is: {}", age);

    //     Punto flotante (f64)
    // Este f64tipo de dato se utiliza para almacenar números que contienen uno o más decimales:

    print!("Numeros flotantes: ");
    let price: f64 = 19.99;
    println!("Price is: ${}", price);

    // Caracteres (char)

    // Este chartipo se utiliza para almacenar un solo carácter. Un valor de tipo char debe ir entre comillas simples, como 'A' o 'c':

    print!("Caracteres: ");
    let my_grade: char = 'B';
    println!("{}", my_grade);

    // Cadenas (&str)

    // Este &strtipo se utiliza para almacenar una secuencia de caracteres (texto).
    //Los valores de cadena deben ir entre comillas dobles:\
    print!("Cadenas: ");
    let name: &str = "Jose";
    println!("Hello, {}!", name);

    // Booleanos (bool)
    // El booltipo solo puede tomar los valores trueo false:

    print!("Booleanos: ");
    let is_logged_in: bool = true;
    println!("User logged in? {}", is_logged_in);

    //Combinación de tipos de datos

    //Puedes mezclar diferentes tipos en el mismo programa:

    let name: &str = "John";
    let age: i32 = 28;
    let is_admin: bool = false;
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Admin: {}", is_admin);

    //
    //Constantes
    //

    /*
         Las variables constantes se utilizan para almacenar valores que nunca cambian.

        A diferencia de las variables regulares, las constantes deben definirse con un tipo (por ejemplo i32, o char).
    */

    //1. Creando una constante

    //Para crear una constante, utilice la constpalabra clave, seguida del nombre, el tipo y el valor:

    const BIRTHYEAR: i32 = 1980;
    const MINUTES_PER_HOUR: i32 = 60;

    /*
        Las constantes deben tener un tipo.
    Debes especificar el tipo al crear una constante.
    No puedes dejar que Rust adivine el tipo como puedes hacerlo con las variables normales:
    */
    print!("Constantes: ");
    println!("Birth year: {}", BIRTHYEAR);
    println!("Minutes per hour: {}", MINUTES_PER_HOUR);

    // 2. Reglas de nomenclatura

    /*  Otro aspecto a tener en cuenta sobre las constantes es que
    se considera una buena práctica declararlas con mayúscula inicial.

    No es obligatorio, pero resulta útil para la legibilidad del código y
    es común entre los programadores de Rust:

    Ejemplos:

    - MAX_SPEED
    - PI
    - MINUTES_PER_HOUR
     */

    //3. Constantes frente a variables

    /*
     Aquí tienes una comparación rápida:
    Feature	        Constant (const)	Variable (let)
    Can change?	        No	                Yes, if mut is used
    Type required?	    Yes	                No (optional)
    */

    //
    // Operadores
    //
    /*
    Los operadores se utilizan para realizar operaciones con valores y variables.

    Rust admite muchos operadores comunes, como:

    - Operadores aritméticos
    - Operadores de asignación
    - Operadores de comparación
    - Operadores lógicos

    */

    //1. Operadores aritméticos

    /*
    Los operadores aritméticos se utilizan para realizar operaciones matemáticas básicas:

        Operator	    Name	               Example	           Result
        +	            Addition	            5 + 3	            8
        -	            Subtraction	            5 - 3	            2
        *	            Multiplication	        5 * 3	            15
        /	            Division	            10 / 2	            5
        %	            Remainder (modulus)	    10 % 3	            1

    ejemplo:
     */

    println!("Operadores: ");

    println!("1. Operadores aritméticos: ");

    let add: i32 = 5 + 3;
    let sub: i32 = 10 - 4;
    let mul: i32 = 6 * 2;
    let div: i32 = 12 / 3;
    let rem: i32 = 10 % 3;

    println!("Add: {}", add);
    println!("Sub: {}", sub);
    println!("Mul: {}", mul);
    println!("Div: {}", div);
    println!("Rem: {}", rem);

    //2. Operadores de asignación

    /*
       Los operadores de asignación se utilizan para asignar y actualizar valores:


       Operator	    Example	        Same As
           =	        x = 5	        Assign 5 to x
           +=	        x += 3	        x = x + 3
           -=	        x -= 2	        x = x - 2
           *=	        x *= 4	        x = x * 4
           /=	        x /= 2	        x = x / 2
           %=	        x %= 2	        x = x % 2

    Ejemplo:
        */
    println!("Operadores de asignación:");
    let mut x: i32 = 10;
    println!("Start: {}", x);

    x += 5;
    println!("After += 5: {}", x);

    x -= 2;
    println!("After -= 2: {}", x);

    x *= 2;
    println!("After *= 2: {}", x);

    x /= 3;
    println!("After /= 3: {}", x);

    x %= 4;
    println!("After %= 4: {}", x);
}
