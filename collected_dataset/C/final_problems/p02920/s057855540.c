#include <stdio.h>

long long max(long long a,long long b){return(a>b)?a:b;}

void swap(long long *a,long long *b){
    long long c;
    c=*a;
    *a=*b;
    *b=c;
}

long long power(int n){
    long long res=1;
    for(long long i=0;i<n;i++){
        res*=2;
    }
    return res;
}

void hsort(long long* a,long long i){
    long long j=i;
    while(j>1){
        if(a[j]>a[j/2]){
            swap(&a[j],&a[j/2]);
            j/=2;
        }
        else break;
    }
}

void dheap(long long* a,long long n){
    long long i=1;
    while(i<=n/2){
        if(a[i*2]>a[i*2+1]){
            swap(&a[i*2],&a[i]);
            i*=2;
        }
        else{
            swap(&a[i],&a[2*i+1]);
            i=i*2+1;
        }
    }
}
        

int judge(long long*a,long long n){
    for(long long i=1;i<n/2;i++){
        if(a[i]<=max(a[2*i],a[2*i+1])){
            return 0;
        }
    }
    return 1;
}

int main(void){
    long long n;
    scanf("%lld",&n);
    n=power(n);
    long long a[n+1];
    for(long long i=1;i<n+1;i++){
        scanf("%lld",&a[i]);
        hsort(a,i);
    }
    if(a[1]>max(a[2],a[3])){
        a[1]=a[n];
        dheap(a,n);
        if(judge(a,n)==0)printf("No\n");
        else printf("Yes\n");
    }
    else printf("No\n");
    return 0;
}
