#include <stdio.h>
int main(void){
    // Your code here!
    int N;
    long long int A[200010];
    int i=0;
    scanf("%d",&N);
    for(i=0;i<N;i++)
    {
        scanf("%lld",&A[i]);
    }
    
    long long int ans=0;
    
    i=0;
    int flag=0; //マイナスが偶数(最後プラスになる)とき
    while(i<N)
    {
        if(A[i]<0)
        {
            ans=ans+(A[i]*(-1));
            if(flag==0) flag=1;
            else        flag=0;
        }
        
        if(A[i]>0)
        {
            ans+=A[i];
        }
        
        if(A[i]==0)
        {
            i++;
            if(A[i]<0)
            {
                ans=ans+(A[i]*(-1));
            }
            else ans+=A[i];
            flag=0;
        }
        

        i++;
        
    }
    
    if(flag==0)
    {
        printf("%lld",ans);
    }
    if(flag==1)
    {
        printf("%lld",ans-(A[N-1]*2));
    }
    return 0;
}
