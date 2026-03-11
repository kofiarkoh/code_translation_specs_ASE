#include <stdio.h>

int main(void){

        int N,K;
        scanf("%d%d",&N,&K);//すぬけn人　お菓子k種類

        int d[100];
        int A[100][100];

for(int i=0;i<K;i++){
        scanf("%d",&d[i]);
        for(int j=0;j<d[i];j++){
                scanf("%d",&A[i][j]);
    
        }   
}

int count[100];

for(int i=0;i<K;i++){
        for(int j=0;j<d[i];j++){//A[i][j]
                for(int a=1;a<=N;a++){
                        if(A[i][j]==a){
                                count[a]++;
                        }   
                }   
        }   
}


int ans = 0;
for(int a=1;a<=100;a++){
        if(count[a]==0){
                ans++;
        }   
}

printf("%d",ans-96);
return 0;

}

