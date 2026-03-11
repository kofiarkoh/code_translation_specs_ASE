#include<stdio.h>
#include<string.h>

int main(void)
{
    char s[16];
  	//char t[16][1];
    int i;
    int ans=0;
    int w=0,l=0,sum=0;
  	
  	scanf("%s",s);

    for(i=0;i<=15;i++){
        if(s[i]=='o'){
            ans+=1;
            w+=1;
          //printf("w=%d\n",w);
        }else if(s[i]=='x'){
            l+=1;
          //printf("l=%d\n",l);
        }
    }
  	//printf("b1:ans=%d\n",ans);

    ans+=16-l;
  	
  	//printf("b2:ans=%d\n",ans);

    if(ans>8){
        puts("YES\n");
    }else{
        puts("NO\n");
    }
    return 0;

}