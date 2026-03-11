#include <stdio.h>
#define N 1000000
int main(void){
    int i,j;
    int n,l;
    int a[N];
    int minnum=-1;
    scanf("%d%d",&n,&l);
    for(i=0;i<n;i++) scanf("%d",&a[i]);
    for(i=0;i<=n-l;i++)
    {
        if(minnum>=i)
        {
            if(a[i+l-1]<=a[minnum]) minnum=i+l-1;
        }
        else
        {
            minnum=i;
            for(j=1;j<l;j++)
            {
               if(a[i+j]<=a[minnum]) minnum=i+j;
            }
        }
        if(i!=0) printf(" ");
        printf("%d",a[minnum]);
    }
    printf("\n");
    return 0;
}
