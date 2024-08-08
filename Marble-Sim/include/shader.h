#ifndef SHADER_H
#define SHADER_H

#include <GL/glew.h>

#include <filesystem>
#include <string>
#include <string_view>

class Shader {
  public:
    Shader(const std::filesystem::path &vertexPath, const std::filesystem::path &fragmentPath);
    void use() const;
    void setBool(const std::string_view &name, bool value) const;
    void setInt(const std::string_view &name, int value) const;
    void setFloat(const std::string_view &name, float value) const;

  private:
    unsigned int ID;
    static std::string readFile(const std::filesystem::path &path);
    static unsigned int compileShader(const char *shaderCode, GLenum shaderType);
    static unsigned int linkProgram(unsigned int vertex, unsigned int fragment);
    static void checkCompileErrors(unsigned int object, std::string_view type);
};

#endif // !SHADER_H
