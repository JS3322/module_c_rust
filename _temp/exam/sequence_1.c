#include <stdio.h>  //printf header file
#include <string.h> //strlen(string length), strcpy(arry a > arry b copy), ctrcmp(a = b > 0, a > b 1..) function header file
#include <math.h>   //pow(a*a*a), sqrt(root), abs(|a|) iclude function herader file
#include <stdlib.h> //atoi(string > int), atof(string > double), srand(nansu gijun), rand(nansu output), time(now time > int)

main()
{
    float i = 0, j = 0;

    do
    {
        i++;
        if ((int)(i / 2) == i / 2)
        {
            j += i / (i + 1);
        }
        else
        {
            j -= i / (i + 1);
        }
    } while (i < 99);
    printf("%f", j);
}