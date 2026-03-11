#include <stdio.h>

int main(void)
{
    int N=0;
    long number[100]={};
    long temp1=0;
    long temp2=0;
    long temp3=0;
    int count=0;
    scanf("%d",&N);
    for(int i=0;i<N;i++){
        scanf("%ld",&number[i]);
    }
    if(N<3){
        printf("0");
        return 0;
    }

    for(int i=0;i<N;i++){
        for(int j=i+1;j<N;j++){
            if(number[i]!=number[j]){
                for(int k=j+1;k<N;k++){
                    if(number[i]!=number[k] && number[j]!=number[k]){
                        temp1=number[i]+number[j]-number[k];
                        temp2=number[j]+number[k]-number[i];
                        temp3=number[k]+number[i]-number[j];
                        if(temp1>0 && temp2>0 && temp3>0){
                            count+=1;
                            //printf("%d %d %d\n",i+1,j+1,k+1);
                        }
                    }
                }
            }
        }
    }

    printf("%d",count);
    return 0;
}
