# include <stdio.h>
# include <string.h>

int main(){
    char str[100005];
    scanf("%s", str);
    int mau = 0;

    char lista[26] = {'a', 'b', 'c', 'd','e','f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n','o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'};
    
    for (int i = 0; i < 26; ++i){
        int dif = 0;
        for (int j = 0; j <strlen(str); ++j){
            if (lista[i] != str[j])
                dif += 1;
        }
        if (dif == strlen(str)){
            printf("%c\n", lista[i]);
            mau += 1;
            break;
        }
    }
    if (mau == 0)
        printf("None\n");
    
    return 0;
}
