#include<stdio.h>
#include<stdlib.h>

#define NEW(p,n){p=malloc((n)*sizeof(p[0]));if(p==NULL){printf("not enough memory\n");exit(1);};}
//pの型の変数n個の要素分のメモリを確保し、そのアドレスをpに代入するマクロ

#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define MIN(a, b) ((a) < (b) ? (a) : (b))

//昇順
int asc(const void* a, const void* b){
    return *(int*)a-*(int*)b;
}

//降順
int desc(const void* a, const void* b){
    return *(int*)b-*(int*)a;
}

int main(void){
    int N,K;
    scanf("%d%d",&N,&K);

    //A[0]~A[N-1]に格納する
    int* x;
    NEW(x,N+1);
    for(int i=0;i<N;i++){
        scanf("%d",&x[i]);
    }
    x[N]=0;
    qsort(x,N+1,sizeof(int),asc);

    int* y;
    NEW(y,N);
    for(int i=0;i<N;i++){
        y[i]=x[i+1]-x[i];
    }

    //スタート地点が、添字何番なのかを格納
    int middle;
    for(int i=0;i<N;i++){
        if(x[i]==0){
            middle=i;
            break;
        }
    }



    long right=0;
    //r,lは次にたすy[i]の候補
    int r=middle;
    int l=middle-1;
    int cnt=0;
    while(cnt<K){
        cnt++;
        if(r==N){
            right+=2*y[l];
            l--;
        }
        else if(l==-1){
            right+=y[r];
            r++;
        }
        else{
            if(y[r]<=2*y[l]){
                right+=y[r];
                r++;
            }
            else{
                right+=2*y[l];
                l--;
            }
        }
    }

    long left=0;
    r=middle;
    l=middle-1;
    cnt=0;
    while(cnt<K){
        cnt++;
        if(r==N){
            left+=y[l];
            l--;
        }
        else if(l==-1){
            left+=2*y[r];
            r++;
        }
        else{
            if(2*y[r]>=y[l]){
                left+=y[l];
                l--;
            }
            else{
                left+=2*y[r];
                r++;
            }
        }
    }

    long ans=MIN(left,right);
    printf("%ld\n",ans);

    return 0;
}