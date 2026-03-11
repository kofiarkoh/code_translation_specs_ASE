#include <stdio.h>

long euclid( long a, long b )
{
    long temp;

    if( a < b ) { temp = a; a = b; b = temp; }
    if( b < 1 ) return -1;

    if( a % b == 0 ) return b;
    return euclid( b, a % b );
}

int main()
{
    int T;
    scanf("%d", &T);
    long a[T], b[T], c[T], d[T];
    int i;
    for(i=0;i<T;i++) scanf("%ld %ld %ld %ld", &a[i], &b[i], &c[i], &d[i]);
    
    for(i=0;i<T;i++){
        if(a[i]<b[i]) printf("No\n");
        else if(b[i]>d[i]) printf("No\n");
        else{
            if(b[i]==d[i]){
                if(a[i]%b[i]>c[i]) printf("No\n");
                else printf("Yes\n");
            }else{
                long sa = d[i]-b[i];
                long amari = a[i]%b[i];
                long left, right;
                left = (c[i]+1-amari)/sa;
                right = (b[i]-1-amari)/sa;
                if(c[i]+1==b[i]-1){
                    if((c[i]+1-amari)%sa==0) printf("No\n");
                    else printf("Yes\n");
                }else if(c[i]+1<b[i]-1){
                   if(amari+sa*left==c[i]+1) printf("No\n");
                   else if(amari+sa*(left+1)<=b[i]-1) printf("No\n");
                   else{
                       if(euclid(sa, b[i])==1) printf("No\n");
                       else printf("Yes\n");
                   }
                }else printf("Yes\n");
            }
        }
    }
    

    return 0;
}
