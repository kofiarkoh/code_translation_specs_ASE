#include <stdio.h>

int main()
{
    int n,tmp=0;
    scanf("%d",&n);
    tmp += n / 25;
    n = n % 25;
    tmp += n / 10;
    n = n % 10;
    tmp += n / 5;
    n = n % 5;
    tmp += n / 1;
    printf("%d\n",tmp);
    return 0;
}
