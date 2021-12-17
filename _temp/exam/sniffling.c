#include <stdio.h>  //printf header file
#include <string.h> //strlen(string length), strcpy(arry a > arry b copy), ctrcmp(a = b > 0, a > b 1..) function header file
#include <math.h>   //pow(a*a*a), sqrt(root), abs(|a|) iclude function herader file
#include <stdlib.h> //atoi(string > int), atof(string > double), srand(nansu gijun), rand(nansu output), time(now time > int)

main()
{
    int i;
    double j;

    i = 0;
    j = 1;
    do
    {
        i++;
        if (i % 2 == 0)
        {
            j *= i;
        }
        else
        {
            j *= i * 1;
        }
    } while (i < 100);
    printf("%11.4e", j);
}