#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<math.h>
#define BUFSIZE 10000000
typedef long long ll;
#define WDSIZE 10000

int A,B,C,K;

int main(void){
    scanf("%d",&A);
    scanf("%d",&B);
    scanf("%d",&C);
    scanf("%d",&K);

    if(K <= A) printf("%d",K);
    else if(K <= A + B){
      printf("%d",A);
    }else printf("%d",A-(K-A-B));

}
