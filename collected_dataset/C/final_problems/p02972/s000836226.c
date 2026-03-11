/* original submission at https://atcoder.jp/contests/abc134/submissions/6473772 
   code by kayaryou */

#include<stdio.h>
int main(void){
    int n, a[200001], b[200001], sum=0;
    scanf("%d", &n);
    for(int i=1; i<=n; i++){
        scanf("%d", &a[i]);
        if(i>n/2){
            b[i]=a[i];
        }
    }
    if(n>2){
        b[1]=-1;
    }
    int m, s;
    for(int i=n/2; i>0; i--){
        m=2*i;
        s=0;
        while(m<=n){
            s+=b[m];
            m=m+i;
        }
        if(s%2==a[i]){
            b[i]=0;
        }else{
            b[i]=1;
        }
    }
    for(int i=1; i<=n; i++){
        sum+=b[i];
    }
    printf("%d\n", sum);
        for(int i=1; i<=n; i++){
            if(b[i]==1){
                printf("%d ", i);
            }
        }
    return 0;

}
