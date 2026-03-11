#include<stdio.h>
long int Co,Co1;
int buffer[101];
int buffer2[101];
int fibonat[10];
int fibo(int n){
    int i=0;
    if(n<=0)return 1;
    if(n==1){
        return 2;
    }else if(n==2){
        return 3;
    }else{
        i=fibo(n-1)+fibo(n-2);
    }
    return i;
}
void Jun(int H,int W,int K,int n){
    buffer[0]=1;
    if(n==H+1){
        if(buffer[H]==K){
            Co=1;
            for(int i=0;i<H;i++){
                if((buffer[i+1]-buffer[i])==1){
                    buffer2[i]=buffer[i];
                }else if((buffer[i+1]-buffer[i])==-1){
                    buffer2[i]=buffer[i]-1;
                }else if((buffer[i+1]-buffer[i])==0){
                    buffer2[i]=0;
                    Co*=fibo(buffer[i]-2)*fibo(W-buffer[i]-1);
                }
            }
            for(int i=0;i<H;i++)Co*=fibonat[buffer2[i]];
            Co1+=Co%1000000007;
        }
    }else{
        if(W==1){
            buffer[n]=1;
            Jun(H,W,K,n+1);
        }else if(W>1){
            if(n==1){
                for(int i=1;i<=2;i++){
                    buffer[n]=i;
                    Jun(H,W,K,n+1);
                }
            }else{
                if(buffer[n-1]==1){
                        for(int i=1;i<=2;i++){
                            buffer[n]=i;
                            Jun(H,W,K,n+1);
                        }
                }else if(buffer[n-1]==W){
                    for(int i=W-1;i<=W;i++){
                        buffer[n]=i;
                        Jun(H,W,K,n+1);
                    }
                }else{
                    for(int i=buffer[n-1]-1;i<=buffer[n-1]+1;i++){
                        buffer[n]=i;
                        Jun(H,W,K,n+1);
                    }
                }
            }
        }
    }
}

void Amida(int H,int W,int K){
    Jun(H,W,K,1);
}

int main(void){
    Co=1;
    Co1=0;
    int H,W,K;
    fibonat[0]=1;
    scanf("%d%d%d",&H,&W,&K);
    for(int i=1;i<=W;i++)fibonat[i]=fibo(i-2)*fibo(W-i-2);
    Amida(H,W,K);
    long int d=Co1%1000000007;
    printf("%ld\n",d);
    return 0;
}

