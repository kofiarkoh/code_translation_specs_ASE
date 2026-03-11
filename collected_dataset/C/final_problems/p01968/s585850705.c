#include <stdio.h>



int main(void){
    int N;
    scanf("%d",&N);
    long long int ans=1;
    int a[N];
    int use[N];
    int kosu=0;
    for(int i=0;i<N;i++)use[i]=0;
    for(int i=0;i<N;i++){
        scanf("%d",&a[i]);
        if(a[i]==2||a[i]==-2){use[i]=1;ans*=a[i];kosu++;}
    }
    if(ans<0){
        int flag=0;
        for(int i=0;i<N;i++){
            if(a[i]==-1){use[i]=1;ans*=-1;kosu++;flag=1;break;}
        }
            if(flag==0){
                for(int i=N-1;i>=0;i--){if(a[i]==-2){use[i]=0;ans/=-2;kosu--;break;}}
            }
        
    }
    printf("%d\n",kosu);
    for(int i=0;i<N;i++){
        if(use[i]==1)printf("%d\n",i+1);
    }
    return 0;
}
