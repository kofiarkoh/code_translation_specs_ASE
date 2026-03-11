#include <stdio.h>
#include <stdlib.h>

int main()
{
    int x,y,res;
    scanf("%d %d",&x,&y);
    if(x==0||y==0){
        if(x==0&&y>0){res=y;
        }else if(x==0&&y<0){res=-y+1;}
        else if(y==0&&x>0){res=x;
        }else{res=-x+1;}
    }else if(abs(x)==abs(y)){res=1;
    }else if(x>0&&y<0){
        if(x>abs(y)){res=x+y+1;
        }else{res=-y-x+1;}
    }
    else if(x<0&&y>0){
        if(abs(x)>y){res=-x-y+1;
        }else{res=y+x+1;}
    }
    else if(x>0&&y>0){
        if(x>y){res=abs(x-y)+1;
        }else{res=abs(x-y);}
    }else{
        if(abs(x)<abs(y)){res=abs(x-y)+2;
        }else{
            res=abs(x-y);
        }
    }
    printf("%d\n",res);
    return 0;
}
