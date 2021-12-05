#include <stdio.h>  //printf header file
#include <string.h> //strlen(string length), strcpy(arry a > arry b copy), ctrcmp(a = b > 0, a > b 1..) function header file
#include <math.h>   //pow(a*a*a), sqrt(root), abs(|a|) iclude function herader file
#include <stdlib.h> //atoi(string > int), atof(string > double), srand(nansu gijun), rand(nansu output), time(now time > int)

main()
{
    int a, j;
    scanf("%d", &a);
    j = 2;
    while (1)
    {
        if (j <= sqrt(a))
        {
            if (a % j == 0)
            {
                printf("not sosu");
                break;
            }
            else
            {
                j++;
            }
        }
        else
        {
            printf("sosu");
            break;
        }
    }
}