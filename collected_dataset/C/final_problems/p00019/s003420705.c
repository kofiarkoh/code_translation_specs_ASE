#include <stdio.h>
double fact(double n)
{
    if(n > 0)
        return n*fact(n-1);
    else 
        return 1;
}
int main()
{
    double num;
    scanf("%lf", &num);
    printf("%.0f",fact(num));

    return 0;
}