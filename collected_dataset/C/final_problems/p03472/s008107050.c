#include <stdio.h>
#include <stdlib.h>
int int_sort_up(const void * a, const void * b)
{
    if(*(int*)a<*(int*)b)
    {
        return -1;
    }
    else if(*(int*)a==*(int*)b)
    {
        return 0;
    }
    return 1;
}
int int_sort_down(const void * a, const void * b)
{
    if(*(long int*)a>*(long int*)b)
    {
        return -1;
    }
    else if(*(long int*)a==*(long int*)b)
    {
        return 0;
    }
    return 1;
}


int main(void)
{
    int n;
    long int h;
    scanf("%d %ld",&n,&h);
    long int a[100003]={};
    long int b[100003]={};
    for(int p=0;p<n;p++)scanf("%ld %ld",&a[p],&b[p]);
    long int time=0;
    qsort(a,n,sizeof(a[0]),int_sort_down);
    qsort(b,n,sizeof(b[0]),int_sort_down);
    int x=0;
    while(h>0&&b[x]>a[0])
    {
        h=h-b[x];
        time++;
        x++;
    }
    if(h>0)
    {
        time+=h/a[0];
        if(h%a[0]!=0)time+=1;
    }
    printf("%ld",time);
}
