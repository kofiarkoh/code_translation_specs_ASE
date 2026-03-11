#include <stdio.h>
#define INF 100001

int main (void){
    int N;
    int a,b;
    int count[INF]={};//テゥツ?催・ツ按療」ツ?ョテヲツ閉ーテ・ツュツ療」ツ?ッテァツァツ?」ツ?凝」ツつ嘉ィツヲツ凝」ツ?淌・ツ渉凝ゥツ?氾」ツ?ョテゥツ?」テ」ツつ古」ツ?ヲティツ。ツ古」ツ?湘、ツコツコテヲツ閉ー
    int i,max=0;
    
    scanf("%d", &N);
    while(N){
        scanf("%d %d",&a ,&b);
        for(a--;a!=b;a++){
           // printf("%d %d\n", a, b);
            count[a]++;
        }
    
        N--;
    }
    for(i=0;i<INF;i++){
        if(count[i] > i)
            count[i] = i;
        if(count[i] < i)
            count[i] = 0;
    }
    
    
    max = count[0];
    for(i=1;i<INF;i++){
        //if(i<10) printf("%d",count[i]);
        if(count[i] > max){
            max = count[i];
        }
    }
    
    printf("%d\n",max);
    return 0;
}