#include <stdio.h>

typedef struct {
    int to;
    int sound;
} EDGE;

int convert(char *str)
{
    int ans;
    
    ans = (str[1] == '#');
    switch (str[0]){
      case 'D':
        ans += 2;
        break;
      
      case 'E':
        ans += 4;
        break;
      
      case 'F':
        ans += 5;
        break;
      
      case 'G':
        ans += 7;
        break;
      
      case 'A':
        ans += 9;
        break;
      
      case 'B':
        ans += 11;
        break;
    }
    
    return (ans);
}

int stairLength, melodyLength;
int stairSound[50000], melodySound[50000];
EDGE graph[50000][3];

int searchPath(int vertex, int cur)
{
    int ans;
    int i;
    
    if (cur == melodyLength && (stairLength - vertex) <= 2){
        return (1);
    }
    
    ans = 0;
    
    if (~vertex){
        for (i = 0; i < 3; i++){
            if (0 <= graph[vertex][i].to && graph[vertex][i].to < stairLength
                    && graph[vertex][i].sound == melodySound[cur]){
                    ans |= searchPath(graph[vertex][i].to, cur + 1);
            }
        }
    }
    else {
        for (i = 0; i < 2; i++){
            if (i < stairLength && (stairSound[i] + i) % 12 == melodySound[cur]){
                ans |= searchPath(i, cur + 1);
            }
        }
    }
    
    return (ans);
}

int main(void)
{
    int n;
    int i, j;
    char in[4];
    
    scanf("%d", &n);
    
    while (n--){
        scanf("%d%d", &stairLength, &melodyLength);
        
        for (i = 0; i < stairLength; i++){
            scanf("%s", in);
            stairSound[i] = convert(in);
        }
        for (i = 0; i < stairLength; i++){
            graph[i][0].to = i + 1;
            if (i + 1 < stairLength){
                graph[i][0].sound = stairSound[i + 1];
            }
            
            graph[i][1].to = i + 2;
            if (i + 2 < stairLength){
                graph[i][1].sound = (stairSound[i + 2] + 1) % 12;
            }
            
            graph[i][2].to = i - 1;
            if (i != 0){
                graph[i][2].sound = (stairSound[i - 1] + 11) % 12;
            }
        }
        
        for (i = 0; i < melodyLength; i++){
            scanf("%s", in);
            melodySound[i] = convert(in);
        }
        
        if (searchPath(-1, 0)){
            printf("Yes\n");
        }
        else {
            printf("No\n");
        }
    }
    
    return (0);
}