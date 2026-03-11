#include<stdio.h>

int main()
{
    int a, b, s1, s2, s3, max;
    scanf("%d %d", &a, &b);
    s1 = a+b;
    s2 = a-b;
    s3 = a*b;
    if(s1>=s2 && s1>=s3)
      max=s1;
    else if(s2>=s1 && s2>=s3)
      max=s2;
    else
      max=s3;
    printf("%d\n", max);
} 