#include <stdio.h>
int prime[50001];
void set(void){
    int i,o;
    for(i=4;i<=5000;i+=2)
        prime[i]=1;
    for(i=3;i*i<=5000;i+=2){
        if(prime[i]==0)
            for(o=i*2;o<=5000;o+=i)
                prime[o]=1;
    }
}
int main(void){
    int num,count,sum,i,o;
    set();
    for(;count=0,scanf("%d",&num),num;){
        for(i=2;i<=num;i++)
            if(!prime[i]){
                if(i>num)
                     break;
                for(o=2;o<=num;o++)
                    if(!prime[o]){
                        sum=i+o;
                        if(sum == num){
                            count++;
                            break;
                        }
                        if(sum > num)
                            break;
                    }
            }
        printf("%d\n",count%2?(count/2+1):(count/2));
    }
    return 0;
}