#include<stdio.h>
int main(void)
{
    int n,i,j,min,temp1,temp2,a[100000],b[100000];
    scanf("%d",&n);
    for(i=0;i<n;i++)
        scanf("%d %d",&a[i],&b[i]);
    for(i=0;i<n-1;i++){
        min=i;
        for(j=i+1;j<n;j++)
            if(a[min]>a[j])
                min=j;
        {temp1=a[min];
        a[min]=a[i];
        a[i]=temp1;
        temp2=b[min];
        b[min]=b[i];
        b[i]=temp2;
        }
    }
    for(i=0;i<n-1;i++){
        if(a[i]+b[i]>a[i+1]-b[i+1]){
            if(a[i]+b[i]>a[i+1]+b[i+1]){
                n--;
            }
            else {
                n--;
                temp1=a[i+1];
                a[i+1]=a[i];
                a[i]=temp1;
                temp2=b[i+1];
                b[i+1]=b[i];
                b[i]=temp2;
            }
        }
    }
    printf("%d",n);
    return 0;
}