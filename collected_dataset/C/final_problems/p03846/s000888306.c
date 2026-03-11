#include<stdio.h>
#include<math.h>
int main(){
    long n;
    scanf("%ld",&n);
    long a[n];
    int ans=1;
    for(int i=0;i<n;i++){
        scanf("%ld",&a[i]);
    }
    int chek=0;
    for(int i=0;i<(n/2)+n%2;i++){
        chek=0;
        if(n%2==1){
            for(int j=0;j<n;j++){
                if(i==0){
                    if(i==a[j])chek+=2;
                }else{
                    if(i*2==a[j])chek+=1;
                }
            }

        }else{
            for(int j=0;j<n;j++){
                if(i*2+1==a[j])chek+=1;
            }
        }
        if(chek!=2){
                    ans=0;
                    break;
                }
    }
    if(ans==0){
        printf("%d\n",ans);
    }else{
        long x=pow(10,9)+7;
        long y=pow(2,(n-n%2)/2);
        y=y%x;
        printf("%ld\n",y);
    }

}