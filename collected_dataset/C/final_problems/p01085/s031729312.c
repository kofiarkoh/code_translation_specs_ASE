#include<stdio.h>
int main(void)
{
int i, j,i_max,n, n_min, n_max, a[1000], sum, max;
 
while(scanf("%d %d %d",&n,&n_min,&n_max))
{
if(n==0&&n_min==0&&n_max==0)
break ;
for(i = 0; i < n ; i++)
{
scanf("%d",&a[i]);
}
max = 0;
for(i = n_min; i <= n_max ; i++)
{
sum = a[i-1]-a[i];
if(sum>=max)
i_max = i;
max = sum;
}
printf("%d\n",i_max);
}
return 0;
}
