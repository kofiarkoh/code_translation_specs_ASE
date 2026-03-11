#include<stdio.h>
#include<stdlib.h>
#define N 10001
#define P 1<<15
typedef long long Int;
Int get_primes(Int primes[P],Int num){
    Int i,j,n=0,flag[P];
    for(i=0;i<num/2;i++) flag[i*2+1] = 1;
    for(i=0;i<num/2;i++) flag[i*2] = 0;
    flag[0] = flag[1] = 0; flag[2] = 1; primes[n++] = 2;
    for(i=3;i<num;i++) if(flag[i]) {
        for(j=2;j<=(num-1)/i;j++) flag[i*j] = 0;
        primes[n++] = i;
    }
    return n;
}
Int pf_decompose(Int num,Int prime_factor[P]){
    Int i,num_pf;
    for(i=2,num_pf=0;i*i<=num;i++){
        if(num%i==0) prime_factor[num_pf++] = i;
        while(num%i==0) num/=i;
    }
    if(num!=1) prime_factor[num_pf++] = num;
    return num_pf;
}
Int gcd(Int a,Int b){ return (a==0)?b:gcd(b%a,a); }
Int Iabs(Int a){ return (a<0)?-a:a; }
int cmp(const void* a,const void* b){ Int c=*(Int*)a,d=*(Int*)b; return (c<d)?-1:(c>d); }
int main(void){
    Int i,j,n,np,m,g,poly[N],tmp[N],primes[P],ans[P];
    scanf("%lld",&n);
    np = get_primes(primes,n+1);
    for(i=n;i>=0;i--) scanf("%lld",&poly[i]);
    for(i=g=0;i<=n;i++) g = gcd(g,Iabs(poly[i]));
    m = pf_decompose(g,ans);
    for(i=0;i<np;i++){
        for(j=0;j<primes[i]-1;j++) tmp[j] = 0;
        for(j=0;j<=n;j++) tmp[j%(primes[i]-1)] += poly[j];
        for(j=0;j<primes[i]-1;j++) if( tmp[j]%primes[i] ) break;
        if(j==primes[i]-1) ans[m++] = primes[i];
    }
    if(m!=0){
        qsort(ans,m,sizeof(Int),cmp);
        printf("%lld\n",ans[0]);
        for(i=1;i<m;i++) if(ans[i]!=ans[i-1]) printf("%lld\n",ans[i]);
    }
    return 0;
}
