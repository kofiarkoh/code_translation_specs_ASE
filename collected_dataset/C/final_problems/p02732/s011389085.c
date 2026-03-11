#include<stdio.h>

int main(void){
    int n,i,B[200003],sum=0,ans=0;
    scanf("%d",&n);
    int A[n+1];
    for(i=1;i<200003;i++){
        B[i] = 0;
    }
    
    for(i=1;i<n+1;i++){
        scanf("%d",&A[i]);
        B[A[i]]++;
    }
    
    for(i=1;i<200003;i++){
        if(B[i]>1){
            sum+= (B[i]*(B[i]-1))/2;
        }
    }
    
    for(i=1;i<n+1;i++){
        ans = sum - B[A[i]] + 1;
        printf("%d\n",ans);
    }
    return 0;
}
