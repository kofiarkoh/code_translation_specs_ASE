#include<stdio.h>
struct data
{
    long long int a;
    long long int b;
    char c;
    long long int d;
    char e[100];
};
int main()
{
    long long int n,i,j;
    scanf("%lld",&n);
    struct data z[n],temp;
    for(i=0;i<n;i++)
    {
        scanf("%lld %lld %c %lld %s",&z[i].a,&z[i].b,&z[i].c,&z[i].d,z[i].e);
    }
    for(i=0;i<n-1;i++)
        for(j=i+1;j<n;j++)
    {
        if(z[i].a>z[j].a)
        {
            temp=z[i];
            z[i]=z[j];
            z[j]=temp;
        }
        if(z[i].a==z[j].a)
        {
            if(z[i].b>z[j].b)
                {
                    temp=z[i];
                    z[i]=z[j];
                    z[j]=temp;
                }
        }
    }
    for(i=0;i<n;i++)
    {
        printf("%lld %lld %c %lld %s\n",z[i].a,z[i].b,z[i].c,z[i].d,z[i].e);
    }
    return 0;
}

