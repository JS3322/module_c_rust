#include <stdio.h>

main()
{
    int q, m;
    int p = 0, h = 0;

    do
    {
        p++;
        q = 78 - p;
        m = q * p;
        h += m;
    } while (p < 77);
    printf("%d", h);
}