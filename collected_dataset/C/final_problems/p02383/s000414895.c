#include<stdio.h>

typedef struct {
    int up;     // 1
    int front;  // 2 
    int right;  // 3
    int left;   // 4
    int back;   // 5
    int down;   // 6
}dice;

dice rollN(dice data);
dice rollE(dice data);
dice rollS(dice data);
dice rollW(dice data);

int main (void){
    dice dice1;
    char ope[100]={0};
    scanf("%d %d %d %d %d %d",&dice1.up,&dice1.front,&dice1.right,&dice1.left,&dice1.back,&dice1.down);
    scanf("%s",ope);
    for (int i = 0; ope[i]!=0; i++)
    {
        switch (ope[i])
        {
        case 'N':
            dice1=rollN(dice1);
            break;
        case 'E':
            dice1=rollE(dice1);
            break;
        case 'S':
            dice1=rollS(dice1);
            break;
        case 'W':
            dice1=rollW(dice1);
            break;
        
        }
    }
    printf("%d\n",dice1.up);
    
    return 0;
}

dice rollN(dice data){
    dice rolleddice;
    rolleddice.up=data.front;
    rolleddice.front=data.down;
    rolleddice.right=data.right;
    rolleddice.left=data.left;
    rolleddice.back=data.up;
    rolleddice.down=data.back;
    return rolleddice;
}

dice rollE(dice data){
    dice rolleddice;
    rolleddice.up=data.left;
    rolleddice.front=data.front;
    rolleddice.right=data.up;
    rolleddice.left=data.down;
    rolleddice.back=data.back;
    rolleddice.down=data.right;
    return rolleddice;
}

dice rollS(dice data){
    dice rolleddice;
    rolleddice.up=data.back;
    rolleddice.front=data.up;
    rolleddice.right=data.right;
    rolleddice.left=data.left;
    rolleddice.back=data.down;
    rolleddice.down=data.front;
    return rolleddice;
}

dice rollW(dice data){
    dice rolleddice;
    rolleddice.up=data.right;
    rolleddice.front=data.front;
    rolleddice.right=data.down;
    rolleddice.left=data.up;
    rolleddice.back=data.back;
    rolleddice.down=data.left;
    return rolleddice;
}
