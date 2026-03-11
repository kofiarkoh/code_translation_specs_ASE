#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <math.h>
#include <stdlib.h>
#include <stdbool.h>
#define MOD 1000000007
void input_array(int how_data,int *data);
int get_random(int min, int max);
int factorial(int n);
int fibonacci(int n);
int qsort_09(const int *sys1 , const int *sys2);
int qsort_90(const int *sys1 , const int *sys2);

long souwa=0;

int main(void){
    int how_card,border;
    scanf("%d %d",&how_card,&border);
    int data[how_card];
    input_array(how_card,data);
    qsort(data,how_card,sizeof(int),(int (*)(const void *,const void *))qsort_09);
    int ans=0;
    int loop1;
    // 2 3 4 10 10 25
    // k=20
    // したから総和見て行って最初に20突破→5番目
    // 1 3 4
    // k=6
    souwa=0;
    loop1=0;
    while(souwa<border){
        souwa+=data[loop1];
        loop1++;
        if(loop1>how_card){
            printf("%d\n",how_card);
            return 0;
        }
    }
    //printf("now=%d wa=%d\n",loop1,souwa);
    int loop2;
    for(loop2=0;loop2<loop1;loop2++){
        if(souwa-data[loop2]>=border)ans++;
    }

    //if(ans==0)ans=how_card;
    printf("%d\n",ans);
    return 0;
    
    
    
}


//how_data個の配列data[]に標準入力
//input_array(how_data,data);
void input_array(int how_data,int *data){
    int loop;
    for(loop=0;loop<how_data;loop++){
        scanf("%d",&data[loop]);
        //souwa+=data[loop];
    }
}

int get_random(int min, int max){   //指定の範囲から乱数を1つ返すやつ
    //srand((unsigned int)time(0));   //現在時刻で疑似乱数初期化
    rand();rand();rand();rand();    //乱数を空打ち
    return (rand()%(max+1-min))+min;
}


int factorial(int n){//n!のMOD10^9+7を返すやつ
    unsigned long long int ret=1;
    for(int i=1;i<=n;i++)ret=(ret*i)%1000000007;
    return (int)ret;
}

int qsort_09(const int *sys1 , const int *sys2){ //小さいのが上にくるやつ
    //qsort(data,how_data,sizeof(int),(int (*)(const void *,const void *))qsort_09);
    if(*sys1<*sys2){
        return -1;
    }else if(*sys1==*sys2){
        return 0;
    }else{
        return 1;
    }
}

int qsort_90(const int *sys1 , const int *sys2){ //大きいのが上にくるやつ
    //qsort(data,how_data,sizeof(int),(int (*)(const void *,const void *))qsort_90);
    if(*sys1<*sys2){
        return 1;
    }else if(*sys1==*sys2){
        return 0;
    }else{
        return -1;
    }
}
int fibonacci(int n){
    switch(n){
        case 0:return 0;
        case 1:return 1;
        default :return fibonacci(n-1)+fibonacci(n-2);
    }
}

