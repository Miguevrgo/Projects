#include <ctype.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/sendfile.h>

#define BUFFER_SIZE 1024

int main()
{
    FILE* fp = fopen("input.txt", "r+");
    if (!fp) {
        perror("File opening failed");
        return EXIT_FAILURE;
    }

    FILE* tmp_file = tmpfile();
    if (!tmp_file) {
        perror("Tmpfile creation failed");
        fclose(fp);
        return EXIT_FAILURE;
    }
    char buffer[BUFFER_SIZE];

    // HACK: This is one way to do it, no big RAM and safe, lets
    // also explore with mmap anc compare efficiency
    while (fgets(buffer, BUFFER_SIZE, fp) != nullptr) {
        for (size_t i = 0; buffer[i] != '\0'; ++i) {
            buffer[i] = (char)tolower((unsigned char)buffer[i]);
        }
        fputs(buffer, tmp_file);
    }

    long pos = ftell(tmp_file);
    if (pos == -1L) {
        perror("Error getting file position indicar for tmp_file");
        fclose(fp);
        fclose(tmp_file);
        return EXIT_FAILURE;
    }
    fseek(tmp_file, 0, SEEK_SET);
    fseek(fp, 0, SEEK_SET);

    // TODO: Validate the state and contents of tmp_file are correct

    int out_fd = fileno(fp);
    int in_fd = fileno(tmp_file);
    if (sendfile(out_fd, in_fd, nullptr, pos) == -1) {
        perror("Error sending file tmp back to input");
        fclose(fp);
        fclose(tmp_file);
        return EXIT_FAILURE;
    }

    fclose(tmp_file);
    fclose(fp);
    return EXIT_SUCCESS;
}
