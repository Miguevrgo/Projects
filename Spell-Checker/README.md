## Spell Checker
# Objetivo
El objetivo es crear un programa en C++ Moderno y eficiente que sirva como un clásico corrector
de palabras al estilo del teclado del movil.

Sugerirá las palabras más probables así como corregirá las palabras mal escritas.

Se basa en una ejecución en la que se selecciona el idioma y se cargan los datos del idioma 
en un TDA Dictionary, el TDA Corrector entonces se ocupa de sugerir las mejores posibles 
correciones para la palabra dada.

Para realizar esta tarea, se calcula la distancia entre la palabra a corregir y sugerencias
mediante una mezcla entre el algoritmo de Levenshtein y mi propio algoritmo que calcula
la distancia de las teclas cambiadas respecto de la palabra por su distancia en un 
teclado QWERTY con distribución ANSI. Esto no deberia afectar mucho ya que QWERTY es ampliamente
utilizado y ANSI no tiene discrencias respecto de ISO en palabras sin caracteres especiales o
numeros.
