#include<stdio.h>
#include<math.h>
int main()
{
    int num,s[200000],i,*p,*q;
    scanf("%d",&num);
   long long d[num-1],x,y,min;
    for(i=0;i<num;i++)
        scanf("%d",&s[i]);
    for(i=0;i<num-1;i++)
    {
        x=0;
        y=0;
        p=s;
        q=s+i+1;
        while(p<q)
            x=x+*p++;
        while(q<s+num)
            y=y+*q++;
        d[i]=fabs(x-y);
    }
     min=d[0];
    for(i=1;i<num-1;i++)
    {
        if(min>d[i])
            min=d[i];
    }
    printf("%lld",min);
}