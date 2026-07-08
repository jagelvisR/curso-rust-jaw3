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

    //3. Operadores de comparación

    /*
    Los operadores de comparación comparan valores y devuelven true o false:

        Operator	    Name	        Example	        Result
        ==	            Equal	        5 == 5	        true
        !=	            Not equal	    5 != 3	        true
        >	            Greater than	5 > 3	        true
        <	            Less than	    5 < 3	        false
        >=	            Greater or equal	5 >= 5	    true
        <=	            Less or equal	5 <= 3	    false

    Ejemplo:
    */
    println!("Operadores de comparación:");
    let a: i32 = 5;
    let b: i32 = 10;

    println!("5 == 10: {}", a == b);
    println!("5 != 10: {}", a != b);
    println!("5 < 10: {}", a < b);
    println!("5 >= 10: {}", a >= b);

    //4. Operadores lógicos

    /*

       Los operadores lógicos se utilizan para trabajar con valores booleanos:


            Operator	        Name	            Description
                &&	            AND	            true if both values are true
                ||	            OR	            true if at least one is true
                !	            NOT	            inverts the boolean value
    Ejemplo:
    */
    let logged_in: bool = true;
    let is_admin: bool = false;

    println!("Is regular user: {}", logged_in && !is_admin);
    println!("Has any access: {}", logged_in || is_admin);
    println!("Not logged in: {}", !logged_in);

    //
    // Operaciones Booleandas en Rust
    //

    /*
    Muy a menudo, en programación, necesitarás un tipo de dato que solo puede tener uno de dos valores, como por ejemplo:

        - SÍ / NO
        - ENCENDIDO / APAGADO
        - VERDADERO / FALSO

    Para ello, Rust dispone de un booltipo de dato conocido como booleano.

    Los booleanos representan valores que son verdadero trueo falso false.

     */

    // Creación de variables booleanas

    /*
    Puedes almacenar un valor booleano en una variable usando el booltipo:

    Ejemplo:
     */
    let is_programming_fun: bool = true;
    let is_fish_tasty: bool = false;

    println!("Is Programming Fun? {}", is_programming_fun);
    println!("Is Fish Tasty? {}", is_fish_tasty);

    /*  Recuerda que Rust es lo suficientemente inteligente como para entender
    que true y false son valores booleanos, lo que significa que no tienes
    que especificar la bool palabra clave:

    let is_programming_fun = true;
    let is_fish_tasty = false;

    println!("Is Programming Fun? {}", is_programming_fun);
    println!("Is Fish Tasty? {}", is_fish_tasty);

     */

    // Booleano de comparación

    /*
    La mayoría de las veces, no es necesario escribirlo trueusted falsemismo.
    En cambio, los valores booleanos provienen de la comparación de valores mediante operadores como ==o >:
    */

    let age: i32 = 20;
    let can_vote: bool = age >= 18;

    println!("Can vote? {}", can_vote);

    /*
    Aquí, age >= 18 devuelve true, siempre y cuando la edad sea de 18 años o más.
     */

    // Uso de booleanos en if sentencias

    /*

    Los valores booleanos se utilizan a menudo en if las sentencias para decidir qué código debe ejecutarse:

    ejemplo:

     */

    let is_logged_in: bool = true;

    if is_logged_in {
        println!("Welcome back!");
    } else {
        println!("Please log in.");
    }

    /*
    Genial, ¿verdad? Los booleanos son la base de todas las comparaciones y condiciones en Rust.
    Aprenderás más sobre las sentencias if y else en el próximo capítulo.

     */

    //
    //  Condiciones IF / ELSE
    //
    println!("Condiciones IF / ELSE: ");
    /*
    Ya sabes que Rust admite condiciones de comparación familiares de las matemáticas, como por ejemplo:

        - Menor que: a < b
        - Menor o igual que: a <= b
        - Mayor que: a > b
        - Mayor o igual que: a >= b
        - Igual a: a == b
        - Distinto de: a != b
    Puedes utilizar estas condiciones para realizar diferentes acciones en función de diferentes decisiones.

    Rust tiene las siguientes sentencias condicionales:

    - Se utiliza if para especificar un bloque de código que se ejecutará si se cumple una condición específica. true
    - Se utiliza else para especificar un bloque de código que se ejecutará si se cumple la misma condición. false
    - Se utiliza else if para especificar una nueva condición a probar, si la primera condición es false
    - Se utiliza match para especificar varios bloques de código alternativos que se ejecutarán.

        Nota: A diferencia de muchos otros lenguajes de programación, if..elseen Rust se puede usar como una instrucción
        o como una expresión (para asignar un valor a una variable). Consulta un ejemplo al final de la página para
        comprenderlo mejor.
    */

    // if
    /*


        Se utiliza ifpara especificar un bloque de código que se ejecutará si se cumple una condición true.

    Ejemplo:

     */
    println!("if statement: ");
    if 7 > 5 {
        println!("7 is greater than 5.");
    }

    // if...else

    /*
    Si la condición no es verdadera, puede usarla elsepara ejecutar un código diferente:

    Ejemplo:

     */
    println!("if...else statement: ");
    let age: i32 = 16;

    if age >= 18 {
        println!("You can vote.");
    } else {
        println!("You are too young to vote.");
    }

    // else if

    /*
    Puedes comprobar varias condiciones usando else if:

    Ejemplo:

     */
    println!("else if statement: ");
    let score: i32 = 85;

    if score >= 90 {
        println!("Grade: A");
    } else if score >= 80 {
        println!("Grade: B");
    } else if score >= 70 {
        println!("Grade: C");
    } else {
        println!("Grade: F");
    }

    // utilizando if como una expresión

    /*
    En Rust, if...elsetambién se puede usar como una expresión .

    Esto significa que puedes asignar el resultado de una operación ifa una variable:

    ejemplo:

    */
    println!("if as an expression: ");
    let time: i32 = 20;
    let greeting: &str = if time < 18 {
        "Good day."
    } else {
        "Good evening."
    };
    println!("{}", greeting);

    /*
    Cuando se utiliza if como expresión, debe incluir else.
    Esto garantiza que el resultado siempre tenga un valor.
     */

    //Sintaxis simplificada de if...else

    /*

    Si cada bloque contiene una sola expresión, puede escribirla de forma más breve en una sola línea:

    Ejemplo:

    let time = 20;
    let greeting = if time < 18 { "Good day." } else { "Good evening." };
    println!("{}", greeting);

     */

    let time: i32 = 20;
    let greeting: &str = if time < 18 {
        "Good day."
    } else {
        "Good evening."
    };
    println!("{}", greeting);

    // No Mezcles tipos

    /*
    Nota: El valor de if y elsedebe ser del mismo tipo, como dos fragmentos de texto o dos números (en el ejemplo anterior, ambos son cadenas de texto).

    Cuando se mezclan tipos, como una cadena y un número entero, se produce un error:

    let number = 5;
    let result = if number < 10 { "Too small" } else { 100 };
    println!("{}", result);

    y no va a compilar. Esto se debe a que el primer bloque devuelve un &str,
    mientras que el segundo bloque devuelve un i32.
    Rust no puede determinar qué tipo de valor debe tener la variable result.

    error[E0308]: `if` and `else` have incompatible types


     */

    //
    // Match
    //
    /*

    Cuando tienes muchas opciones, usar matches más fácil que escribir mucho if...else.

    match Se utiliza para seleccionar uno de los muchos bloques de código que se ejecutarán:

    */

    println!("match statement: ");

    let day: i32 = 4;

    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day."),
    }

    /*
        Ejemplo explicado:

    - La match variable ( día ) se evalúa una sola vez.
    - El valor de la variable día se compara con los valores de cada "rama".
    - Cada rama comienza con un valor, seguido de =>y un resultado.
    - Si hay una coincidencia, se ejecuta el bloque de código asociado.
    - _ Se utiliza para especificar algún código que se ejecutará si no hay coincidencia
    (como default en otros lenguajes).
    - En el ejemplo anterior, el valor de día es 4 , lo que significa que se imprimirá "Thursday".

        */

    // Multiples coincidencias

    println!("match statement with multiple matches: ");

    let day: i32 = 6;

    match day {
        1 | 2 | 3 | 4 | 5 => println!("Weekday"),
        6 | 7 => println!("Weekend"),
        _ => println!("Invalid day"),
    }

    //match con un valor de retorno
    println!("match statement with return value: ");
    /*
    Al igual que if, matchtambién puede devolver un valor:

    Esto significa que puedes guardar el resultado de una coincidencia en una variable:

    ejemplo:

     */

    let day: i32 = 4;

    let result: &str = match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Invalid day.",
    };

    println!("{}", result);

    /*
        Nota: Cada parte de las matchramas debe ser del mismo tipo , igual que con if...else.
    */

    //
    // Rust Loops
    //
    println!("Rust Loops: ");

    //Loops
    println!("Loops statement: ");

    /*
    Los bucles pueden ejecutar un bloque de código siempre que se cumpla una condición específica.

    Los bucles son muy útiles porque ahorran tiempo, reducen errores y hacen que el código sea más legible. Por ejemplo, en lugar de escribir la misma línea diez veces para imprimir un texto, puedes usar un bucle para que lo repita automáticamente.

    Rust tiene tres tipos de bucles: loop, while, y for.

     */

    // Loop
    println!("Loop statement: ");
    /*
    loop Es el más simple de los tres tipos de bucles de Rust.

      Se ejecutará indefinidamente a menos que le indiques que se detenga:

      loop {
        println!("This will repeat forever!");
    }

    Advertencia: ¡Este bucle nunca se detiene! Deberá presionar Ctrl + C para finalizar el programa.

    Para detener un bucle, utilice la breakpalabra clave:

      */

    let mut count: i32 = 1;

    loop {
        println!("Hello World!");

        if count == 3 {
            break;
        }

        count += 1;
    }

    /*
    Ejemplo explicado:

        Esto imprime "¡Hola Mundo!" 3 veces.
        Utiliza un contador para llevar la cuenta de cuántas veces se ha repetido el bucle.
        El contador comienza en 1 ( let mut count = 1;).
        Cada vez que se ejecuta el bucle, el contador aumenta en 1: ( count += 1;).
        Cuando llega a 3, el bucle se detiene.

     */

    // Devuelve un valor
    println!("Loop statement with return value: ");

    /*
        También puede devolver un valor desde un loopuso breakcon un valor.

    Esto te permite guardar el resultado del bucle en una variable:

    Ejemplo:
         */

    let mut count: i32 = 1;

    let result: i32 = loop {
        println!("Hello!");

        if count == 3 {
            break count; // Stop the loop and return the number 3
        }

        count += 1;
    };

    println!("The loop stopped at: {}", result);

    /*
        Este bucle imprime "¡Hola!" hasta que count llega a 3, luego se detiene y devuelve ese número.

        Nota: Cuando guardes el resultado de un bucle en una variable, debes poner un punto y coma ( ;) al final.

        A continuación: Aprende a usar whilebucles para repetir código mientras se cumpla una condición.

    */

    // While Loops

    println!("While Loops statement: ");

    /*
    El whilebucle se ejecuta mientras se cumpla una condición true.

     */

    let mut count: i32 = 1;

    while count <= 5 {
        println!("Count: {}", count);
        count += 1;
    }

    //Detener un bucle While
    //Puedes detener un whilebucle cuando quieras usando break:
    println!("While Loops statement with break: ");

    let mut count: i32 = 1;

    while count <= 5 {
        println!("Count: {}", count);

        if count == 3 {
            break; // Stop the loop when count is 3
        }

        count += 1;
    }

    /*
        Este bucle imprime "Count: 1", "Count: 2" y "Count: 3", luego se detiene.

        Esto imprime los números del 1 al 5 (el bucle se detiene cuando num llega al 6).

    A continuación: Aprende a usar el forbucle para recorrer un rango de valores.

         */

    //Saltar un valor

    println!("While Loops statement with continue: ");

    let mut num = 1;

    while num <= 10 {
        if num == 6 {
            num += 1;
            continue;
        }

        println!("Number: {}", num);
        num += 1;
    }
    /*
        Esta impresora imprime los números del 1 al 10, excepto el número 6.

    A continuación: Aprende a usar el forbucle para recorrer un rango de valores.
         */

    //El forLoop
    println!("For Loops statement: ");

    /*
    Cuando sepa exactamente cuántas veces desea recorrer un bloque de código,
    utilice el forbucle junto con la inpalabra clave, en lugar de un whilebucle:
     */

    for i in 1..6 {
        println!("i is: {}", i);
    }

    /*
        Imprime los números del 1 al 5.

    Nota: 1..6 significa desde 1 hasta (pero sin incluir) 6.

    Nota: Rust gestiona la variable contador ( i) automáticamente, a diferencia de muchos otros lenguajes de programación. No es necesario declararla ni incrementarla manualmente.

    */

    //Gama inclusiva

    println!("Inclusive range in For Loops statement: ");

    //Si desea incluir el último número, utilice ..=(dos puntos y un signo de igual):

    for i in 1..=6 {
        println!("i is: {}", i);
    }

    //Esta impresora imprime los números del 1 al 6, incluido el 6.

    //Descanso y continuación

    println!("Break and Continue in For Loops statement: ");

    /*
    Al igual que con otros bucles, puedes usar break para detener el bucle y continue para omitir un valor:
    */

    for i in 1..=10 {
        if i == 3 {
            continue; // skip 3
        }
        if i == 5 {
            break; // stop before printing 5
        }
        println!("i is: {}", i);
    }

    /*
    Esto imprime 1, 2 y 4. Se salta el 3 y se detiene antes del 5.
     */

    //Resumen de bucles de Rust

    /*  Rust tiene tres tipos de bucles que permiten ejecutar código repetidamente.
    Cada uno se utiliza en situaciones diferentes: */

    //1.loop
    /*El tipo de bucle más simple.
    Se ejecuta indefinidamente a menos que lo detengas con break.

    loop {
        // do something
        if condition {
            break;
        }
    }
     */

    //2.while
    println!("While Loops statement: ");
    /* Repite el código mientras una condición sea verdadera.
    Comprueba la condición antes de cada iteración del bucle. */

    let mut count: i32 = 0;
    while count < 5 {
        println!("Count: {}", count);
        count += 1;
    }

    /* Úsalo while cuando quieras repetir el código hasta que ocurra algo. */

    //3.for
    println!("For Loops statement: ");
    /* Repite el código un número fijo de veces. */

    for i in 1..=5 {
        println!("{}", i);
    }

    /* Úselo for cuando sepa exactamente qué elementos recorrer en el bucle. */

    /*
    Palabras clave adicionales
        Puedes utilizarlos en cualquier bucle:

        break- detener el bucle

        continue- omitir un valor en el bucle

     */

    //
    //Funciones de Rust
    //
    println!("Funciones de Rust: ");

    //funciones

    /*
        Una función es un bloque de código que solo se ejecuta cuando se la llama.

        Las funciones se utilizan para organizar el código, evitar repeticiones
        y facilitar la comprensión del programa.

    */

    //Creación de una función

    /*
            Para crear una función, utilice la fn palabra clave,
            seguida del nombre de la función y un conjunto de paréntesis () y llaves {}:

            ejemplo:

            fn function_name() {
                // code to be executed
            }

    */

    //Llamando a una función

    /*
    Ahora que has creado una función, puedes ejecutarla llamándola .

    Para llamar a una función, escriba el nombre de la función seguido de dos paréntesis ().

    ejemplo:
    */

    // Create a function
    fn say_hello() {
        println!("Hello from a function!");
    }

    say_hello(); // Call the function

    //Funciones con parámetros
    println!("Funciones con parámetros: ");

    /*
    Puedes enviar información a una función usando parámetros.
    Los parámetros se escriben dentro de los paréntesis ().
    */

    fn greet(name: &str) {
        println!("Hello, {}!", name);
    }

    greet("Jose"); // Call the function with a parameter

    /*En este ejemplo, la función toma un parámetro de cadena llamado name y lo imprime en el mensaje de saludo. */

    //Funciones con valores de retorno
    println!("Funciones con valores de retorno: ");

    /*

        Una función también puede devolver un valor.

        Utilice el  -> símbolo en el encabezado de la función para indicar qué tipo de valor se devolverá.

        Dentro de la función, utilice la return palabra clave para devolver el valor:

           fn add(a: i32, b: i32) -> i32 {
               return a + b;
    }

    let sum = add(3, 4);
    println!("Sum is: {}", sum);
    */
    fn addf(a: i32, b: i32) -> i32 {
        return a + b;
    }
    let sum = addf(3, 4);
    println!("Sum is: {}", sum);

    /*
    Esta función suma dos números y devuelve el resultado.

    En Rust, puedes omitir la return palabra clave.
    Simplemente escribe el valor en la última línea de la función,
    sin punto y coma :

        */

    println!("Funciones con valores de retorno sin return: ");

    fn addsr(a: i32, b: i32) -> i32 {
        a + b
    }

    let sum = addsr(3, 4);
    println!("Sum is: {}", sum);

    /*
            La última línea a + bse devuelve automáticamente.

        Ambos ejemplos hacen lo mismo. Tú decides cuál usar.


        ¿Por qué usar funciones?
        - Para organizar tu código
        - Para evitar repetir el mismo código
        - Para que sus programas sean más fáciles de leer y modificar.

    */

    //
    //Scope
    //
    println!("Scope: ");

    /*
        Ahora que ya entiendes cómo funcionan las funciones,
        es importante aprender cómo se comportan las variables
        dentro y fuera de ellas.

        El ámbito se refiere a dónde se permite el uso de una variable.

        Una variable solo existe dentro del bloque donde fue creada.

        Un bloque es todo lo que está dentro de las llaves { }.

    */

    // variable dentro de una función
    println!("Variable dentro de una función: ");
    /*
        Una variable creada dentro de una función solo existe dentro de esa función:


        fn myFunction() {
            let message = "Hello!";
            println!("{}", message); // You can access the message variable here
        }

        myFunction();

        println!("{}", message);// Error - you cannot access the message variable outside of the function


        Nota: La variable message solo existe dentro de la función.
        Intentar usarla fuera de la función provocará un error.
    */

    // Variable dentro de un bloque
    println!("Variable dentro de un bloque: ");
    /*
        También puedes crear bloques dentro de otro código,
        como en if sentencias o bucles.
        Las variables creadas en estos bloques solo son válidas dentro de ellos.

        Ejemplo:

    */

    let score: i32 = 80;

    if score > 50 {
        let result = "Pass";
        println!("Result: {}", result);
    }

    println!("Result: {}", result); // Error: result is out of scope here

    // Variables en el mismo ambito

    /*
    En Rust, puedes declarar una nueva variable con el mismo nombre en el
    mismo ámbito usando let. Esto se llama sombreado :

    Ejemplo:
    */

    let x: i32 = 5;
    let x: i32 = x + 1;
    println!("The value of x is: {}", x);

    /*
       La segunda x reemplaza a la primera.
       El valor 5 ya no es accesible después de la segunda declaración.

       Esto difiere de los lenguajes que no permiten reutilizar nombres de variables.
       En Rust, es una característica que se utiliza para transformar
       o actualizar valores de forma segura.

       También puedes reutilizar el nombre de una variable dentro de
       un nuevo bloque.

       Ejemplo:

    */

    let x: i32 = 5;

    {
        let x: i32 = 10;
        println!("Inside block: {}", x);
    }

    println!("Outside block: {}", x);

    /*
        Aquí, las dos x variables se encuentran en ámbitos diferentes.

        La variable interna x solo existe dentro del bloque.

        Fuera del bloque, se mantiene su valor original.

        Nota: Si bien se permite el uso de nombres repetidos,
        usar el mismo nombre con demasiada frecuencia puede dificultar
        la lectura del código.
        Utilice nombres claros siempre que sea posible.

        Por qué el scope es importante

        Comprender el scope te ayuda a:

        Saber dónde se puede usar una variable

        Evitar conflictos de nombres

        Evite errores al trabajar con funciones, bucles y condicionales.

    */

    //
    // Strings
    //
    println!("Strings: ");
    /*
       Tambien conocidas como cadenas de texto,
       Las cadenas de texto se utilizan para almacenar texto.

       Ya has aprendido que puedes usar este &str tipo para crear una cadena

       Ejemplo:

    */

    let greeting: &str = "Hello";
    println!("{}", greeting);

    /*
    Ten en cuenta que las cadenas están rodeadas de comillas dobles ( " Hola " ).

    En Rust existen dos tipos principales de cadenas de caracteres:

    &str - se denomina "segmentos de cadena" y se utiliza para texto fijo como "Hello"
    String - se utiliza cuando se necesita una cadena que pueda cambiar

    En este capítulo, trabajarás principalmente con este String tipo de datos
    porque es más flexible y se puede modificar con el tiempo.
     */

    //Crear una cadena de texto
    println!("Crear una cadena de texto: ");
    /*

    Puedes crear un String a partir de un literal de cadena usando
    el to_string() método o la String::from() función

    Ejemplo:
    */

    let text1c: String = "Hello World".to_string();
    println!("Crear una cadena de texto con .to_string(): {}", text1c);

    let text2c: String = String::from("Hello Rust");
    println!("Crear una cadena de texto con String::from(): {}", text2c);

    //cambiar una cadena
    println!("Cambiar una cadena de texto: ");

    /*
       Las cadenas son mutables, por lo que puedes cambiarlas si se declaran con mut.

       Se utiliza push_str() para añadir texto a una cadena:


    */
    println!("Cambiar una cadena de texto con push_str(): ");
    let mut greeting: String = String::from("Hello");
    greeting.push_str(" World");
    println!("{}", greeting); // Hello World

    /*
        Se utiliza push() para añadir un carácter:

        Ejemplo:
    */
    println!("Cambiar una cadena de texto con push(): ");
    let mut word: String = String::from("Hi");
    word.push('!');
    println!("{}", word); // Hi!
}
