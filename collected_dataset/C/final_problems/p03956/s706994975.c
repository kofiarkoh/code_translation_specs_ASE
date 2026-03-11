//Nます　10^5
//x,y==y,z

#include<stdio.h>
#include<stdlib.h>

//0が白、1が黒

int main(){
    int N,M,x,y,z,i,j,k,count,*Map;
    
    scanf("%d %d",&N,&M);

    Map=(int *)calloc(N*N,sizeof(int));
    if(Map==NULL){
        printf("メモリが確保できません\n");
        exit(-1);
    }

    for(k=0;k<M;k++){
        scanf("%d %d",&i,&j);
        *(Map+(N*(i-1))+(j-1)) = 1;
    }

    for(x=0;x<N;x++){
        for(y=0;y<N;y++){
            if(*(Map+(N*x)+y)==0) continue;
            for(z=0;z<N;z++){
                if(*(Map+(N*y)+z)==0) continue;
                if(*(Map+(N*z)+x)==0) *(Map+(N*z)+x)=1;
            }
        }
    }

    count = 0;
    for(x=0;x<N;x++){
        for(y=0;y<N;y++){
            if(*(Map+(N*x)+y)==1)count++;
        }
    }

    printf("%d\n",count);

    free(Map);
    return 0;
}