#include <stdio.h>

#define MAX_N 100000
int tmp[MAX_N];

void func(int Len,long long *head){
    int cent = Len/2;
    int tip  = Len%2;
    if(Len > 2){
        func(cent, head);
        func(cent+tip, head+cent);
    }
    int i,j,k;
    i = k = 0;
    j = cent;
    if(Len == 1){
        tmp[k] = head[0];
        k++;
    }else{
        while(i < cent && j < Len){
            if(head[i] > head[j]){
                tmp[k] = head[i];
                k++;
                i++;
            }else{
                tmp[k] = head[j];
                k++;
                j++;
            }
        }

        while(i < cent){
            tmp[k] = head[i];
            k++;
            i++;
        }

        while(j < Len){
            tmp[k] = head[j];
            k++;
            j++;
        }
    }
    int l;
    for(l = 0;l < k;l++){
        head[l] = tmp[l];
    }
    return;
}

int main(void){
    int N;
    int A[MAX_N],B[MAX_N];
    long long int AB[MAX_N];
    scanf(" %d",&N);
    int i;
    for(i = 0;i < N;i++){
        scanf(" %d %d",&A[i],&B[i]);
        AB[i] = A[i] + B[i];
    }
    func(N,AB);
    long long int bsum = 0;
    for(i = 0;i < N;i++){
        bsum += B[i];
    }
    long long int ans = 0;
    for(i = 0;i < N;i+=2){
            ans += AB[i];
    }
    ans = ans - bsum;
    printf("%lld\n",ans);
    return 0;
}