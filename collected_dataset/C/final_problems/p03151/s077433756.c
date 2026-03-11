#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <stdio.h>
int compare(const void *a,const void *b){
    if(*(long long *)a>*(long long *)b)return -1;
     return 1;
}
int comp(const void *a,const void *b){
    if(*(long long *)a>*(long long *)b)return 1;
     return -1;
}
int main()
{   int i=0,n;
    scanf("%d",&n);
    long long a[n],b[n],d[n],husoku=0,m=0,stock=0,o[n];
    for(i=0;i<n;i++){
        scanf("%lld",&a[i]);
        
    }
    for(i=0;i<n;i++){
        scanf("%lld",&b[i]);
        if(a[i]<b[i]){
            husoku+=b[i]-a[i];
            m++;
            d[i]=a[i]-b[i];
        }else{
            stock+=a[i]-b[i];
            d[i]=a[i]-b[i];
        }
        o[i]=d[i];
    }
    if(stock<husoku){
        printf("-1");
        return 0;
    }
    if(husoku==0){
        printf("0");
        return 0;
    }
    int ans=1,k=0;
    qsort(d,n,sizeof(long long),compare);
    qsort(o,n,sizeof(long long),comp);
    for(i=0;i<n;i++){
        if(o[i]<0&&d[k]+o[i]>=0){
            ans++;
            d[k]+=o[i];
        }else if(o[i]<0&&d[k]+o[i]<0){
            o[i]+=d[k];
            ans+=2;
            k++;
            d[k]-=o[i];
        }
    }
    printf("%d",ans);
    return 0;
}
