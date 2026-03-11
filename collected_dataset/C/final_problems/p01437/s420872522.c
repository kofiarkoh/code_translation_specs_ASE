#include <stdio.h>


int main()
{
    while(1)
    {
        int H,W,x,y; long long int L;
        scanf("%d %d %lld",&H,&W,&L);
        if(H==0) break;
        int i,j; char dir, grid[100][100];
        for(i=0; i<H; i++) scanf("%s",&grid[i]);
        for(i=0; i<H; i++)
            for(j=0; j<W; j++)
                if(grid[i][j]!='#' && grid[i][j]!='.')
                {
                    dir=grid[i][j];
                    x=i;y=j;
                }
        grid[x][y]='.';
        for(i=0; i<L; i++)
        {
            switch(dir){

            case 'E':
                {
                    if(grid[x][y+1]=='.') y++;
                    else if(grid[x+1][y]=='.')
                    {
                        x++;
                        dir='S';
                    }
                    else if(grid[x][y-1]=='.')
                    {
                        x--;
                        dir='W';
                    }
                    else
                    {
                        y--;
                        dir='N';
                    }
                    break;
                }
            case 'W':
                {
                    if(grid[x][y-1]=='.') y--;
                    else if(grid[x-1][y]=='.')
                    {
                        x--;
                        dir='N';
                    }
                    else if(grid[x][y+1]=='.')
                    {
                        y++;
                        dir='E';
                    }
                    else
                    {
                        x++;
                        dir='S';
                    }
                    break;
                }
            case 'N':
                {
                    if(grid[x-1][y]=='.') x--;
                    else if(grid[x][y+1]=='.')
                    {
                        y++;
                        dir='E';
                    }
                    else if(grid[x+1][y]=='.')
                    {
                        x++;
                        dir='S';
                    }
                    else
                    {
                        x--;
                        dir='W';
                    }
                    break;
                }
            case 'S':
                {
                    if(grid[x+1][y]=='.') x++;
                    else if(grid[x][y-1]=='.')
                    {
                        y--;
                        dir='W';
                    }
                    else if(grid[x-1][y]=='.')
                    {
                        x--;
                        dir='N';
                    }
                    else
                    {
                        y++;
                        dir='E';
                    }
                    break;
                }
            default: return 0;
            }

        }
        printf("%d %d %c\n",x+1,y+1,dir);


    }
    return 0;
}