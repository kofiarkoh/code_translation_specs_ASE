#include<stdio.h>
#include<string.h>
#include<stdlib.h>
#include<ctype.h>
char digit[4]={'m','c','x','i'};
void to_num(int nums[4],char s[10]){
    for(int j=0;j<strlen(s);j++){
        char c=s[j];
        switch(c){
            case 'm':
                if(j==0) nums[0]=1;
                if(j-1<0) break;
                if(isdigit(s[j-1])) nums[0]=s[j-1]-'0';
                else nums[0]=1;
                break;
            case 'c':
                if(j==0) nums[1]=1;
                if(j-1<0) break;
                if(isdigit(s[j-1])) nums[1]=s[j-1]-'0';
                else nums[1]=1;
                break;
            case 'x':
                if(j==0) nums[2]=1;
                if(j-1<0) break;
                if(isdigit(s[j-1])) nums[2]=s[j-1]-'0';
                else nums[2]=1;
                break;
            case 'i':
                if(j==0) nums[3]=1;
                if(j-1<0) break;
                if(isdigit(s[j-1])) nums[3]=s[j-1]-'0';
                else nums[3]=1;
                break;
            default : break;
        }
    }
}
void sum(int nums[],int numt[],int ans[]){
    for(int i=0;i<4;i++){
        ans[i]=nums[i]+numt[i];
    }
    for(int i=3;i>=0;i--){
        if(ans[i]>9){
            ans[i]=ans[i]%10;
            ans[i-1]++;
        }
    }
}
int main(void){
    int n;
    scanf("%d",&n);
    getchar();
    for(int i=0;i<n;i++){
        char s[10],t[10];
        scanf("%s%s",s,t); getchar();
        //printf("|%s %s|\n",s,t); printf("%lu\n",strlen(s));
        int nums[4]={0},numt[4]={0};
        to_num(nums,s);
        to_num(numt,t);
        //for(int j=0;j<4;j++) printf("%d",nums[j]); puts("");
        //for(int j=0;j<4;j++) printf("%d",numt[j]); puts("");
        int ans[4]={0};
        sum(nums,numt,ans);
        //for(int j=0;j<4;j++) printf("%d",ans[j]); puts("");
        for(int j=0;j<4;j++){
            if(ans[j]>1){
                printf("%d%c",ans[j],digit[j]);
            }
            else if(ans[j]==1){
                printf("%c",digit[j]);
            }
        }puts("");
    }
}

